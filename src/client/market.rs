use super::*;
use crate::error::*;
use crate::models::*;

use serde_json::from_str;

impl Client {
    pub fn common_symbols(&self) -> APIResult<Vec<Pair>> {
        let data = self.get("/v1/common/symbols", "")?;
        let response: APIResponse<Vec<Pair>> = from_str(data.as_str()).unwrap();

        Ok(response.data)
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
