use crate::schema::*;

#[derive(Debug, Queryable)]
pub struct Resource {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Insertable)]
#[table_name = "resource_"]
pub struct NewResource<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Queryable)]
pub struct Item {
    pub id: String,
    pub icon_link: Option<String>,
    pub wiki_link: Option<String>,
    pub image_link: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "item_"]
pub struct NewItem<'a> {
    pub id: &'a str,
    pub icon_link: Option<&'a str>,
    pub wiki_link: Option<&'a str>,
    pub image_link: Option<&'a str>,
}

#[derive(Debug, Queryable)]
pub struct Trader {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "trader_"]
pub struct NewTrader<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Queryable)]
pub struct PriceData {
    pub item_id: String,
    pub timestamp: i32,
    pub base_price: i32,
    pub avg_24h_price: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "price_data_"]
pub struct NewPriceData<'a> {
    pub item_id: &'a str,
    pub timestamp: i32,
    pub base_price: i32,
    pub avg_24h_price: Option<i32>,
}

#[derive(Debug, Queryable)]
pub struct TraderPriceData {
    pub item_id: String,
    pub trader_id: String,
    pub timestamp: i32,
    pub price: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "trader_price_data_"]
pub struct NewTraderPriceData<'a> {
    pub item_id: &'a str,
    pub trader_id: &'a str,
    pub timestamp: i32,
    pub price: i32,
}
