use crate::schema::*;

#[derive(Debug, Queryable)]
pub struct Resource {
    pub key: String,   // Primary key
    pub value: String, //
}

#[derive(Debug, Insertable)]
#[table_name = "resource"]
pub struct NewResource<'a> {
    pub key: &'a str,   // Primary key
    pub value: &'a str, //
}

#[derive(Debug, Queryable)]
pub struct MarketItem {
    pub uid: String,       // Primary key
    pub slots: i32,        //
    pub wiki_link: String, //
    pub img_link: String,  //
}

#[derive(Debug, Insertable)]
#[table_name = "market_item"]
pub struct NewMarketItem<'a> {
    pub uid: &'a str,       // Primary key
    pub slots: i32,         //
    pub wiki_link: &'a str, //
    pub img_link: &'a str,  //
}

#[derive(Debug, Queryable)]
pub struct MarketItemName {
    pub uid: String,        // Compound key part 1
    pub lang: String,       // Compound key part 2
    pub name: String,       //
    pub short_name: String, //
}

#[derive(Debug, Insertable)]
#[table_name = "market_item_name"]
pub struct NewMarketItemName<'a> {
    pub uid: &'a str,        // Compound key part 1
    pub lang: &'a str,       // Compound key part 2
    pub name: &'a str,       //
    pub short_name: &'a str, //
}

#[derive(Debug, Queryable)]
pub struct PriceData {
    pub uid: String,             // Compound key part 1
    pub timestamp: i32,          // Compound key part 2
    pub price: i32,              //
    pub avg_24h_price: i32,      //
    pub avg_7d_price: i32,       //
    pub trader_name: String,     //
    pub trader_price: i32,       //
    pub trader_currency: String, //
}

#[derive(Debug, Insertable)]
#[table_name = "price_data"]
pub struct NewPriceData<'a> {
    pub uid: &'a str,             // Compound key part 1
    pub timestamp: i32,           // Compound key part 2
    pub price: i32,               //
    pub avg_24h_price: i32,       //
    pub avg_7d_price: i32,        //
    pub trader_name: &'a str,     //
    pub trader_price: i32,        //
    pub trader_currency: &'a str, //
}
