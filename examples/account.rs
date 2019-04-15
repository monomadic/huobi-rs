extern crate huobi;
use huobi::*;

fn main() {
    let client = Client::new("YOUR_API_KEY", "YOUR_SECRET_KEY");

    match client.accounts() {
        Ok(accounts) => println!(
            "accounts:\n{}",
            accounts
                .into_iter()
                .map(|account| format!(
                    "{}: {} - {}",
                    account.account_id, account.state, account.account_type
                ))
                .collect::<Vec<String>>()
                .join("\n")
        ),
        Err(why) => println!("{}", why),
    }
}
