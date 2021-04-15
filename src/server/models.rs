#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Item {
    pub id: String,
    #[serde(rename = "iconLink")]
    pub icon_link: Option<String>,
    #[serde(rename = "wikiLink")]
    pub wiki_link: Option<String>,
    #[serde(rename = "imageLink")]
    pub image_link: Option<String>,
    #[serde(flatten)]
    pub price_data: PriceData,
    #[serde(rename = "traderPrices")]
    pub trader_prices: Vec<TraderPriceData>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PriceData {
    pub timestamp: i32,
    #[serde(rename = "basePrice")]
    pub base_price: i32,
    #[serde(rename = "avg24hPrice")]
    pub avg_24h_price: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TraderPriceData {
    #[serde(rename = "traderId")]
    pub trader_id: String,
    pub timestamp: i32,
    pub price: i32,
}
