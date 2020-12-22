use super::models::MarketItem;
use std::sync::Arc;

#[derive(Clone)]
pub struct StateHandle {
    inner: Arc<State>,
}

impl StateHandle {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(State {
                market_items: Default::default(),
                market_items_gzip: Default::default(),
            }),
        }
    }
}

impl std::ops::Deref for StateHandle {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

pub struct State {
    market_items: dashmap::DashMap<String, Vec<MarketItem>>,
    market_items_gzip: dashmap::DashMap<String, Vec<u8>>,
}

impl State {
    pub fn update_from_db(
        &self,
        languages: &[&str],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let conn = super::get_db_connection()?;

        for lang in languages {
            self.update(&conn, lang)?;
        }

        Ok(())
    }

    fn update(
        &self,
        conn: &diesel::SqliteConnection,
        lang: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let market_items = {
            use crate::schema::*;
            use diesel::prelude::*;

            let mut market_items = Vec::new();

            for market_item in market_item::table.load::<crate::db::models::MarketItem>(conn)? {
                let mut price_data = match price_data::table
                    .filter(price_data::uid.eq(&market_item.uid))
                    .order(price_data::timestamp.desc())
                    .first::<crate::db::models::PriceData>(conn)
                    .optional()?
                {
                    Some(pd) => super::models::PriceData {
                        timestamp: pd.timestamp,
                        price: pd.price,
                        avg_24h_price: pd.avg_24h_price,
                        avg_7d_price: pd.avg_7d_price,
                        avg_24h_ago: 0,
                        avg_7d_ago: 0,
                        trader_name: pd.trader_name,
                        trader_price: pd.trader_price,
                        trader_currency: pd.trader_currency,
                    },
                    None => continue,
                };

                // Get average from 24h ago
                match price_data::table
                    .filter(price_data::uid.eq(&market_item.uid))
                    .filter(price_data::timestamp.gt(price_data.timestamp - 60 * 60 * 24))
                    .order(price_data::timestamp.asc())
                    .first::<crate::db::models::PriceData>(conn)
                    .optional()?
                {
                    Some(pd) => {
                        price_data.avg_24h_ago = pd.avg_24h_price;
                    }
                    None => {}
                }

                // Get average from 7 days ago
                match price_data::table
                    .filter(price_data::uid.eq(&market_item.uid))
                    .filter(price_data::timestamp.gt(price_data.timestamp - 60 * 60 * 24 * 7))
                    .order(price_data::timestamp.asc())
                    .first::<crate::db::models::PriceData>(conn)
                    .optional()?
                {
                    Some(pd) => {
                        price_data.avg_7d_ago = pd.avg_7d_price;
                    }
                    None => {}
                }

                let market_item_name = match market_item_name::table
                    .filter(market_item_name::uid.eq(&market_item.uid))
                    .filter(market_item_name::lang.eq(lang))
                    .first::<crate::db::models::MarketItemName>(conn)
                    .optional()?
                {
                    Some(market_item_name) => market_item_name,
                    None => {
                        match market_item_name::table
                            .filter(market_item_name::uid.eq(&market_item.uid))
                            .filter(market_item_name::lang.eq("en"))
                            .first::<crate::db::models::MarketItemName>(conn)
                            .optional()?
                        {
                            Some(market_item_name) => market_item_name,
                            None => continue,
                        }
                    }
                };

                market_items.push(super::models::MarketItem {
                    uid: market_item.uid,
                    name: market_item_name.name,
                    short_name: market_item_name.short_name,
                    slots: market_item.slots,
                    wiki_link: market_item.wiki_link,
                    img_link: market_item.img_link,
                    price_data,
                });
            }

            market_items
        };

        // Compress data
        {
            use flate2::write::GzEncoder;
            use std::io::Write;

            let compression_level = flate2::Compression::best();

            let mut e = GzEncoder::new(Vec::new(), compression_level);

            let data = serde_json::to_string(&market_items)?;
            let data = data.as_bytes();
            e.write_all(data)?;
            let compressed_data = e.finish()?;

            self.market_items_gzip
                .insert(lang.to_string(), compressed_data);
        }

        self.market_items.insert(lang.to_string(), market_items);
        Ok(())
    }

    pub fn get(&self, lang: Option<&str>) -> Option<Vec<u8>> {
        match lang {
            Some(lang) => match self.market_items_gzip.get(lang) {
                Some(market_items_gzip) => Some(market_items_gzip.clone()),
                None => self.market_items_gzip.get("en").map(|v| v.value().clone()),
            },
            None => self.market_items_gzip.get("en").map(|v| v.value().clone()),
        }
    }
}
