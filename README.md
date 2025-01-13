## Why?
I want flight pricing data and I don't want to pay for it. There are a zillion services that offer flight pricing APIs but they all cost money. Google dropped it's flights API in 2018 but you can still get the data if you're willing to get creative 😈

### Ok, but flight pringing statistics are already pretty widely available?
Sometimes it's just more fun to roll your own.


## Running locally
- Install Rust and Cargo
- `cargo run`

### What happened?
For each source/destination pair of the top 10 busiest airports in th US, the program scrapes goolge flights for the cheapest ticket price for the next 330 days and saves it to a sqlite database located in the root of the project.
If run every day, the db will slowly reveal the optimal time of year to buy a flight, and the optimal time in advance a flight should be purchased.
