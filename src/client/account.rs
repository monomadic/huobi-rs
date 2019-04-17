use super::*;
use serde_json::from_str;

impl Client {
    pub fn accounts(&self) -> APIResult<Vec<Account>> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let data = self.get_signed("/v1/account/accounts", params)?;
        let response: APIResponse<Vec<Account>> = from_str(data.as_str())?;
        Ok(response.data)
    }

    pub fn balance(&self, id: u32) -> APIResult<Balance> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let data = self.get_signed(&format!("/v1/account/accounts/{}/balance", id), params)?;
        let response: APIResponse<Balance> = from_str(data.as_str())?;
        Ok(response.data)
    }

    pub fn orders(&self, symbol: &str) -> APIResult<Vec<Order>> {
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert(
            "staet".to_string(),
            "pre-submitted,submitted,partial-filled,partial-canceled".to_string(),
        );
        params.insert("types".to_string(), "buy-limit".to_string());
        let data = self.get_signed("/v1/order/orders", params)?;

        let response: APIResponse<Vec<Order>> = from_str(data.as_str())?;
        Ok(response.data)
    }
}
