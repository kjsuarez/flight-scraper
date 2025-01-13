use rusqlite::{params, Connection, Result, ToSql,};
use crate::scraper;

pub struct TicketsTable {
    connection: Connection
}

impl TicketsTable {
    pub fn init() -> TicketsTable {
        let conn = Connection::open("tickets.db").unwrap();
        conn.execute(
            "CREATE TABLE if not exists tickets (
                purchase_date TEXT NOT NULL,
                flight_date TEXT NOT NULL,
                origin TEXT NOT NULL,
                destination TEXT NOT NULL,
                price INTEGER NOT NULL,
                days_in_advance INTEGER NOT NULL,
                PRIMARY KEY (purchase_date, flight_date, origin, destination)
            )",
            (),
        ).unwrap();

        TicketsTable {
            connection: conn
        }
    }

    pub fn test(&self){
        let query = "INSERT OR IGNORE INTO tickets (
            purchase_date, 
            flight_date, 
            origin, 
            destination, 
            price, 
            days_in_advance
        ) VALUES (?, ?, ?, ?, ?, ?), (?, ?, ?, ?, ?, ?)"; 

        let mut statement = self.connection.prepare_cached(query).unwrap();
        statement.execute(rusqlite::params!["1111111", "2", "3", "4", 5, 6, "77777777", "8", "9", "fart", 11, 12]);
    }

    pub fn batch_insert(&self, batch: Vec<scraper::Ticket>) {
        let mut query = "INSERT OR IGNORE INTO tickets (
            purchase_date, 
            flight_date, 
            origin, 
            destination, 
            price, 
            days_in_advance
        ) VALUES ";
        let mut values: String = " (?, ?, ?, ?, ?, ?),".repeat(batch.len() as usize);
        values.pop();
        let value_str = values.as_str();
        let final_query = format!("{}{}",query, value_str);

        let mut params: Vec<_> = Vec::new();
        // populate params
        for ticket in batch.iter(){
            params.push(&ticket.purchase_date as &dyn ToSql);
            params.push(&ticket.flight_date as &dyn ToSql);
            params.push(&ticket.origin as &dyn ToSql);
            params.push(&ticket.destination as &dyn ToSql);
            params.push(&ticket.price as &dyn ToSql);
            params.push(&ticket.days_in_advance as &dyn ToSql);
        }

        let mut statement = self.connection.prepare_cached(&final_query).unwrap();
        statement.execute(&*params).unwrap();

    }

    pub fn insert(&self, ticket: &scraper::Ticket) {
        self.connection.execute(
            "INSERT OR IGNORE INTO tickets (
                purchase_date, 
                flight_date, 
                origin, 
                destination, 
                price, 
                days_in_advance
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                ticket.purchase_date.clone(), 
                ticket.flight_date.clone(), 
                ticket.origin.clone(), 
                ticket.destination.clone(), 
                ticket.price, 
                ticket.days_in_advance),
        ).unwrap();
    }

    pub fn count(&self) -> u32 {
        let mut stmt = self.connection.prepare("SELECT COUNT(*) FROM tickets").unwrap();
        stmt.query_row([], |r| r.get(0)).unwrap()
    }

}

