#[derive(Debug, serde::Deserialize)]
pub enum Response {
    #[serde(rename = "data")]
    Data {
        #[serde(rename = "itemsByType")]
        items_by_type: Vec<Item>,
    },
    #[serde(rename = "errors")]
    Error {},
}

#[derive(Debug, serde::Deserialize)]
pub struct Item {
    pub id: String,
    #[serde(rename = "basePrice")]
    pub base_price: i64,
    pub updated: Option<String>,
    #[serde(rename = "iconLink")]
    pub icon_link: Option<String>,
    #[serde(rename = "wikiLink")]
    pub wiki_link: Option<String>,
    #[serde(rename = "imageLink")]
    pub image_link: Option<String>,
    #[serde(rename = "lastLowPrice")]
    pub last_low_price: Option<i64>,
    #[serde(rename = "low24hPrice")]
    pub low_24h_price: Option<i64>,
    #[serde(rename = "avg24hPrice")]
    pub avg_24h_price: Option<i64>,
    #[serde(rename = "traderPrices")]
    pub trader_prices: Vec<TraderPrice>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TraderPrice {
    pub price: i64,
    pub trader: Trader,
}

#[derive(Debug, serde::Deserialize)]
pub struct Trader {
    pub id: String,
    pub name: String,
}
