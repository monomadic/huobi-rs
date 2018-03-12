extern crate kucoin;
use kucoin::*;

fn main() {
    let client = Client::new("YOUR_API_KEY", "YOUR_SECRET_KEY");
    println!("{:?}", client.balances());
}
