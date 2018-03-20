use models::*;
use client::*;
use error::*;

use serde_json::from_str;

impl Client {
    pub fn balances(&self) -> Result<Vec<Balance>, KucoinError> {
        let data = self.get_signed("/v1/account/balances", "")?;
        let response:APIResponse<Balances> = from_str(data.as_str())?;
        // println!("{:?}", data);

        Ok(response.data.balances)
    }

    pub fn prices(&self) -> Result<Vec<Coin>, KucoinError> {
        let data = self.get_signed("/v1/market/symbols", "")?;
        let response:APIResponse<Vec<Coin>> = from_str(data.as_str())?;
        // println!("{:?}", data);

        Ok(response.data)
    }
}
