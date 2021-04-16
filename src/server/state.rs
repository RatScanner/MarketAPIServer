use super::models::Item;
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
    pub async fn update_from_db(
        &self,
        languages: &[&str],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut conn = super::get_db_connection().await?;

        for lang in languages {
            self.update(&mut conn, lang).await?;
        }

        Ok(())
    }

    async fn update(
        &self,
        conn: &mut sqlx::SqliteConnection,
        lang: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let items = {
            let mut items = Vec::new();

            for item in sqlx::query!("SELECT * FROM item_")
                .fetch_all(&mut *conn)
                .await?
            {
                let price_data = sqlx::query!(
                    r#"
                    SELECT * FROM price_data_
                    WHERE item_id = ?
                    ORDER BY timestamp DESC
                    "#,
                    item.id
                )
                .map(|record| super::models::PriceData {
                    timestamp: record.timestamp,
                    base_price: record.base_price,
                    avg_24h_price: record.avg_24h_price,
                })
                .fetch_optional(&mut *conn)
                .await?;

                let price_data = match price_data {
                    Some(price_data) => price_data,
                    None => continue,
                };

                let trader_prices = sqlx::query!(
                    r#"
                    SELECT * FROM trader_price_data_
                    WHERE item_id = ?
                    GROUP BY trader_id
                    HAVING MAX(timestamp)
                    ORDER BY timestamp DESC
                    "#,
                    item.id
                )
                .map(|record| super::models::TraderPriceData {
                    trader_id: record.trader_id,
                    timestamp: record.timestamp,
                    price: record.price,
                })
                .fetch_all(&mut *conn)
                .await?;

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