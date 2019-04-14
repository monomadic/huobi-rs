extern crate huobi;
use huobi::*;

fn main() {
    let client = Client::new("YOUR_API_KEY", "YOUR_SECRET_KEY");

    println!("symbols: {:?}", client.common_symbols());

    match client.accounts() {
        Ok(accounts) => println!("accounts: {:?}", accounts),
        Err(why) => println!("error: {}", why),
    }
}
