use super::*;
use crate::error::*;
use crate::models::*;

use serde_json::from_str;

impl Client {
    pub fn accounts(&self) -> APIResult<Vec<Account>> {
        println!("ffaaaaf");
        let data = self.get_signed("/v1/account/accounts", "")?;

        //        println!("fff{:?}", data);
        let response: APIResponse<Vec<Account>> = from_str(data.as_str())?;

        println!("{:?}", response);

        Ok(response.data)
    }
}
