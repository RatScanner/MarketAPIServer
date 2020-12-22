#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MarketItem {
    #[serde(rename = "uid")]
    pub uid: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "shortName")]
    pub short_name: String,

    #[serde(rename = "slots")]
    pub slots: i32,

    #[serde(rename = "wikiLink")]
    pub wiki_link: String,

    #[serde(rename = "imgLink")]
    pub img_link: String,

    #[serde(flatten)]
    pub price_data: PriceData,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PriceData {
    #[serde(rename = "timestamp")]
    pub timestamp: i32,

    #[serde(rename = "price")]
    pub price: i32,

    #[serde(rename = "avg24hPrice")]
    pub avg_24h_price: i32,

    #[serde(rename = "avg7dPrice")]
    pub avg_7d_price: i32,

    #[serde(rename = "avg24hAgo")]
    pub avg_24h_ago: i32,

    #[serde(rename = "avg7dAgo")]
    pub avg_7d_ago: i32,

    #[serde(rename = "traderName")]
    pub trader_name: String,

    #[serde(rename = "traderPrice")]
    pub trader_price: i32,

    #[serde(rename = "traderCurrency")]
    pub trader_currency: String,
}
