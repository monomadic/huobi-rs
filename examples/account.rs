extern crate huobi;
use huobi::*;

fn main() {
    let client = Client::new("YOUR_API_KEY", "YOUR_SECRET_KEY");
    println!("{:?}", client.common_symbols());
}
