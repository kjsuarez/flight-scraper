use rusqlite::{params, Connection, Result};
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
        // stmt.query_row([]).unwrap()
        // stmt.query_row::<u32,_>( &[], |r| r.get(0)).unwrap()
        stmt.query_row([], |r| r.get(0)).unwrap()
    }

}

