use super::*;
use crate::error::*;
use crate::models::*;

use serde_json::from_str;

impl Client {
    pub fn common_symbols(&self) -> APIResult<Vec<Pair>> {
        let data = self.get("/v1/common/symbols", "")?;
        let response: APIResponse<Vec<Pair>> = from_str(data.as_str())?;

        Ok(response.data)
    }

    pub fn common_currencys(&self) -> APIResult<Vec<Currency>> {
        let data = self.get("/v1/common/currencys", "")?;
        let response: APIResponse<Vec<Currency>> = from_str(data.as_str())?;

        Ok(response.data)
    }

    pub fn common_timestamp(&self) -> APIResult<Timestamp> {
        let data = self.get("/v1/common/timestamp", "")?;
        let response: APIResponse<Timestamp> = from_str(data.as_str())?;

        Ok(response.data)
    }
}
