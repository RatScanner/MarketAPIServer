use super::models::Item;
use crate::schema::*;
use diesel::prelude::*;
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
    market_items: dashmap::DashMap<String, Vec<Item>>,
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
        let items = {
            let mut items = Vec::new();

            for item in item_::table.load::<crate::db::models::Item>(conn)? {
                let price_data = match price_data_::table
                    .filter(price_data_::item_id.eq(&item.id))
                    .order(price_data_::timestamp.desc())
                    .first::<crate::db::models::PriceData>(conn)
                    .optional()?
                {
                    Some(pd) => super::models::PriceData {
                        timestamp: pd.timestamp,
                        base_price: pd.base_price,
                        avg_24h_price: pd.avg_24h_price,
                    },
                    None => continue,
                };

                // TODO: ...
                let trader_prices = Vec::new();
                /*let trader_prices = trader_price_data_::table
                .filter(trader_price_data_::item_id.eq(&item.id))
                .order(trader_price_data_::timestamp.desc())
                .load::<crate::db::models::TraderPriceData>(conn)?
                .into_iter()
                .map(|tpd| super::models::TraderPriceData {
                    trader_id: tpd.trader_id,
                    timestamp: tpd.timestamp,
                    price: tpd.price,
                })
                .collect();*/

                items.push(super::models::Item {
                    id: item.id,
                    icon_link: item.icon_link,
                    wiki_link: item.wiki_link,
                    image_link: item.image_link,
                    price_data,
                    trader_prices,
                });
            }

            items
        };

        // Compress data
        {
            use flate2::write::GzEncoder;
            use std::io::Write;

            let compression_level = flate2::Compression::best();

            let mut e = GzEncoder::new(Vec::new(), compression_level);

            let data = serde_json::to_string(&items)?;
            let data = data.as_bytes();
            e.write_all(data)?;
            let compressed_data = e.finish()?;

            self.market_items_gzip
                .insert(lang.to_string(), compressed_data);
        }

        self.market_items.insert(lang.to_string(), items);
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
