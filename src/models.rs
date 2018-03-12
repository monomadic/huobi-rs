
#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse<R> {
    pub success: bool,
    pub code: String,
    pub data: R,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTime {
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub phase: String,
    pub revision: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balances {
    pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub currency: String,       // Currency ID "BTC", "ETH"
    #[serde(rename = "type")]
    pub currency_type: String,  // enum[exchange]
    pub total: f64,             // Total amount of balance
    pub on_order: f64,
    pub locked: bool,
}
