use chrono::{Days, Local, NaiveDate};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize};
#[derive( Debug)]
pub struct ScrapedFlightData {
    pub tickets: Vec<Ticket>,
}
impl ScrapedFlightData {
    
    pub fn tickets_from_post_resp(origin: String, destination: String, data: String) -> Vec<Ticket> {
        let re = Regex::new(r##"(\d{4}-\d{2}-\d{2})\\",null,..null,(\d{1,4})"##).unwrap();
        let mut tickets = Vec::new();

        let today = Local::now().date_naive();
        let today_str = format!("{}", today.format("%Y-%m-%d"));
    
    
        for (_, [date_str, price]) in re.captures_iter(&data).map(|c| c.extract()) {
            let today = NaiveDate::parse_from_str(&today_str, "%Y-%m-%d").unwrap();
            let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
            tickets.push( 
                Ticket { 
                    purchase_date: today_str.clone(), 
                    flight_date: date_str.to_string(), 
                    days_in_advance: (date - today).num_days() as u32,
                    origin: origin.to_string(), 
                    destination: destination.to_string(), 
                    price: price.parse::<u32>().unwrap()
                }
            );
        }

        tickets
    }
}

#[derive( Debug)]
pub struct Ticket {
    purchase_date: String, 
    flight_date: String,
    days_in_advance: u32,
    origin: String,
    destination: String,
    price: u32,
}

#[derive( Debug)]
pub struct Query {
    start_date: String,
    end_date: String, 
    origin: String,
    destination: String,
}
impl Query {
    pub const PAGE_LENGTH: u8 = 100;

    fn body(&self) -> String {
        String::from(format!("f.req=%5Bnull%2C%22%5Bnull%2C%5Bnull%2Cnull%2C2%2Cnull%2C%5B%5D%2C1%2C%5B1%2C0%2C0%2C0%5D%2Cnull%2Cnull%2Cnull%2Cnull%2Cnull%2Cnull%2C%5B%5B%5B%5B%5B%5C%22{0}%5C%22%2C0%5D%5D%5D%2C%5B%5B%5B%5C%22{1}%5C%22%2C0%5D%5D%5D%2Cnull%2C0%5D%5D%2Cnull%2Cnull%2Cnull%2C1%5D%2C%5B%5C%22{2}%5C%22%2C%5C%22{3}%5C%22%5D%5D%22%5D&at=ALvc21ZWkqlCKEHrH7_bXxOJ7i0A%3A1736227763751&", self.origin, self.destination, self.start_date, self.end_date))
    }

    fn standard() -> Query {
        let today = Local::now().date_naive();
        let tommorow = today + Days::new(200);
        Query {
            start_date: format!("{}", today.format("%Y-%m-%d")),
            end_date: format!("{}", tommorow.format("%Y-%m-%d")),
            origin: String::from("ORD"),
            destination: String::from("PHL"),
        }   
    }

    fn paginated(origin: &str, destination: &str, page: u64) -> Query {
        let draw_distance: u64 = 329;
        let upper_limit = Local::now().date_naive() + Days::new(draw_distance);
        let page_length = 100;
        let start = Local::now().date_naive() + Days::new(page_length * (page - 1));
        let mut end = Local::now().date_naive() + Days::new((page_length * page) - 1);
        if end > upper_limit {
            end = upper_limit
        }
        Query {
            start_date: format!("{}", start.format("%Y-%m-%d")),
            end_date: format!("{}", end.format("%Y-%m-%d")),
            origin: String::from(origin),
            destination: String::from(destination),
        }  
    }
}


pub fn get_raw_flight_data(query: Query, client: &reqwest::blocking::Client) ->  String {
    const GOOGLE_FLIGHTS_CALENDAR_URL: &str = "https://www.google.com/_/FlightsFrontendUi/data/travel.frontend.flights.FlightsFrontendService/GetCalendarPicker";
    let scrape_res = client.post(GOOGLE_FLIGHTS_CALENDAR_URL)
    .header("Content-Type", "application/x-www-form-urlencoded;charset=utf-8")
    .body(query.body())
    .send().unwrap();

    scrape_res.text().unwrap()
}



// scrape all possible data for a source destination pair
pub fn scrape( origin: &str, destination: &str) -> ScrapedFlightData {
    let client = reqwest::blocking::Client::new();

    let mut working = true;
    let mut page:u64 = 1;
    let mut results = ScrapedFlightData {
        tickets: Vec::new()
    };
    while working {
        let query = Query::paginated(origin, destination, page);
        println!("Query: {:?}", query);
        let resp = get_raw_flight_data(query, &client);
        println!("response: {resp:?}");

        let tickets = &mut ScrapedFlightData::tickets_from_post_resp(
            String::from("PHL"), 
            String::from("ORD"), 
            resp
        );

        if tickets.len() < (Query::PAGE_LENGTH - 1) as usize {
            println!("ticket count: {} vs page length: {}", tickets.len(), Query::PAGE_LENGTH - 1);
            working = false;
        }

        results.tickets.append( tickets );
        page += 1;
    }
    results
}

 