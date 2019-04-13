use client::*;
use error::*;
use models::*;

use serde_json::from_str;

impl Client {
    pub fn common_symbols(&self, symbol: &str) -> APIResult<String> {
        let data = self.get("/v1/common/symbols", "")?;
        let response: APIResponse<ServerTime> = from_str(data.as_str()).unwrap();

        Ok(response.result.time)
    }
//
//    /// get server time
//    pub fn get_server_time(&self) -> APIResult<u64> {
//        let data = self.get("/v1/system/time", "")?;
//        let response: APIResponse<ServerTime> = from_str(data.as_str()).unwrap();
//
//        Ok(response.result.time)
//    }
//
//    pub fn get_server_info(&self) -> APIResult<ServerInfo> {
//        let data = self.get("/v1/system/info", "")?;
//        let response: APIResponse<ServerInfo> = from_str(data.as_str()).unwrap();
//
//        Ok(response.result)
//    }
}
