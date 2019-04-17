extern crate huobi;
use huobi::*;

fn main() {
    let client = Client::new("YOUR_API_KEY", "YOUR_SECRET_KEY");

    match client.common_symbols() {
        Ok(pairs) => println!(
            "symbols: {}",
            pairs
                .into_iter()
                .map(|pair| pair.symbol)
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Err(why) => println!("error: {}", why),
    }

    println!("common_timestamp: {:?}", client.common_timestamp());
}
