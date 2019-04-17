use super::*;
use crate::error::*;
use crate::models::*;
use serde_json::from_str;

impl Client {
    pub fn accounts(&self) -> APIResult<Vec<Account>> {
        let data = self.get_signed("/v1/account/accounts", "")?;
        let response: APIResponse<Vec<Account>> = from_str(data.as_str())?;
        Ok(response.data)
    }

    pub fn balance(&self, id: u32) -> APIResult<Balance> {
        let data = self.get_signed(&format!("/v1/account/accounts/{}/balance", id), "")?;
        println!("data: {:?}", data);
        let response: APIResponse<Balance> = from_str(data.as_str())?;
        Ok(response.data)
    }
}
