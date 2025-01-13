mod db;
mod scraper;
use crate::db::api;

fn all_combos(set: Vec<&str>) -> Vec<(&str, &str)> {
    let mut combos = Vec::new();
    for i in set.iter() {
        for j in set.iter() {
            if i != j {
                combos.push((*i, *j));
            }
        }
    }
    combos
}

fn main() {

    println!("initializing db...");
    let ticket_db = api::TicketsTable::init();
    
    let airports = vec![
    "ATL", "DFW", 
    "DEN", "LAX", 
    "ORD", "JFK", 
    "MCO", "LAS", 
    "CLT", "MIA"];

    let combos = all_combos(airports);
    let flight_count = combos.len();
    println!("Scraping google flights...");
    for (i, flight) in combos.iter().enumerate() {
        // println!("scapeing {} out of {}", i, flight_count);

        let mut results = scraper::scrape(flight.0, flight.1);
        
        let mut batch_size = 0;
        let mut bucket = Vec::new();
        while &results.tickets.len() > &0 {
            batch_size = if results.tickets.len() > 50 {
                50
            } else {
                results.tickets.len()
            };
            bucket = results.tickets.split_off(batch_size);

            ticket_db.batch_insert(results.tickets.clone());
            results.tickets = bucket;
        }
        // println!("table size before insert:{}", ticket_db.count());
        // println!("updating db...");
        for ticket in results.tickets.iter() {
            ticket_db.insert(ticket);
        }
        // println!("table size after insert:{}", ticket_db.count());
    }
    
    
}

