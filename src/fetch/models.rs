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
    pub base_price: i32,
    pub updated: Option<String>,
    #[serde(rename = "iconLink")]
    pub icon_link: Option<String>,
    #[serde(rename = "wikiLink")]
    pub wiki_link: Option<String>,
    #[serde(rename = "imageLink")]
    pub image_link: Option<String>,
    #[serde(rename = "avg24hPrice")]
    pub avg_24h_price: Option<i32>,
    #[serde(rename = "traderPrices")]
    pub trader_prices: Vec<TraderPrice>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TraderPrice {
    pub price: i32,
    pub trader: Trader,
}

#[derive(Debug, serde::Deserialize)]
pub struct Trader {
    pub id: String,
    pub name: String,
}
