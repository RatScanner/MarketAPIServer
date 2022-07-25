#[derive(Debug, serde::Deserialize)]
pub enum Response {
    #[serde(rename = "data")]
    Data {
        #[serde(rename = "items")]
        items: Vec<Item>,
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
    #[serde(rename = "avg24hPrice")]
    pub avg_24h_price: Option<i64>,
    #[serde(rename = "lastLowPrice")]
    pub last_low_price: Option<i64>,
    #[serde(rename = "sellFor")]
    pub sell_for: Vec<SellFor>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SellFor {
    pub price: i64,
    pub vendor: Vendor,
}

#[derive(Debug, serde::Deserialize)]
pub struct Vendor {
    pub name: String,
    pub trader: Option<Trader>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Trader {
    pub id: String,
    pub name: String,
}
