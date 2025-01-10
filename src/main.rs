mod scraper;

fn main() {
    let mut results = scraper::scrape("PHL", "ORD");

    println!("out: {results:?}");
}

