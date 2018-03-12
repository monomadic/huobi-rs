use models::*;
use client::*;
use error::*;

use serde_json::from_str;

impl Client {
    pub fn balances(&self) -> Result<Vec<Balance>, KucoinError> {
        let data = self.get("/account/balances", "")?;
        let response:APIResponse<Vec<Balance>> = from_str(data.as_str()).unwrap();

        Ok(response.data)
    }
}
