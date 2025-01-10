mod db;
mod scraper;
use crate::db::api;

fn main() {
    
    const AIRPORTS: [&str; 10] = [
    "ATL", "DFW", 
    "DEN", "LAX", 
    "ORD", "JFK", 
    "MCO", "LAS", 
    "CLT", "MIA"];

    println!("Scraping google flights...");
    let results = scraper::scrape("PHL", "ORD");
    
    println!("initializing db...");
    let ticket_db = api::TicketsTable::init();
    

    
    println!("table size before insert:{}", ticket_db.count());
    println!("updating db...");
    for ticket in results.tickets.iter() {
        ticket_db.insert(ticket);
    }
    println!("table size after insert:{}", ticket_db.count());

    
    // println!("out: {results:?}");
}

