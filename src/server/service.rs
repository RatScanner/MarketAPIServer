use super::state::StateHandle;
use crate::schema::*;
use diesel::prelude::*;

pub fn start(state: StateHandle) {
    std::thread::spawn(move || {
        loop {
            let state = std::panic::AssertUnwindSafe(state.clone());

            let service_result = std::panic::catch_unwind(move || run(state.0));

            // Restart service on crash
            match service_result {
                Ok(()) => break,
                Err(_) => log::error!("Service crashed. Restarting ..."),
            }
        }
    });
}

fn run(state: StateHandle) {
    let languages = ["en"]; // ["en", "ru", "de", "fr", "es", "cn"];
    let mut languages_cycle = languages.iter().cycle();

    loop {
        // Fetch and update
        let res = async_std::task::block_on(async {
            fetch_and_update(languages_cycle.next().unwrap()).await
        });

        match res {
            Ok(_) => {
                if let Err(e) = state.update_from_db(&languages) {
                    log::error!("failed to update from db: {}", e);
                }
                std::thread::sleep(std::time::Duration::from_secs(60 * 10));
            }
            Err(e) => {
                log::error!("failed to fetch and update: {}", e);
                std::thread::sleep(std::time::Duration::from_secs(60));
            }
        }
    }
}

async fn fetch_and_update(
    _lang: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use diesel::prelude::*;

    // Fetch
    let timestamp_fallback = chrono::Utc::now().timestamp() as i32;
    let items = crate::fetch::fetch().await?;

    // Write to db
    let conn = super::get_db_connection()?;

    conn.transaction::<_, Box<dyn std::error::Error + Send + Sync + 'static>, _>(|| {
        for item in items {
            // Calc timestamp
            let timestamp = match &item.updated {
                Some(_) => {
                    // TODO: ...
                    timestamp_fallback
                }
                None => timestamp_fallback,
            };

            // Upsert ...
            upsert_item(&conn, &item)?;
            upsert_price_data(&conn, &item, timestamp)?;
            for trader_price in &item.trader_prices {
                upsert_trader_price_data(&conn, &item.id, trader_price, timestamp)?;
            }
        }
        Ok(())
    })?;

    Ok(())
}

fn upsert_item(
    conn: &diesel::sqlite::SqliteConnection,
    item: &crate::fetch::models::Item,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let exists = item_::table
        .filter(item_::id.eq(&item.id))
        .count()
        .get_result::<i64>(conn)?
        != 0;

    if exists {
        diesel::update(item_::table)
            .filter(item_::id.eq(&item.id))
            .set((
                item_::icon_link.eq(&item.icon_link),
                item_::wiki_link.eq(&item.wiki_link),
                item_::image_link.eq(&item.image_link),
            ))
            .execute(conn)?;
    } else {
        diesel::insert_into(item_::table)
            .values(&crate::db::models::NewItem {
                id: &item.id,
                icon_link: item.icon_link.as_deref(),
                wiki_link: item.wiki_link.as_deref(),
                image_link: item.image_link.as_deref(),
            })
            .execute(conn)?;
    }

    Ok(())
}

fn upsert_price_data(
    conn: &diesel::sqlite::SqliteConnection,
    item: &crate::fetch::models::Item,
    timestamp: i32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let exists = price_data_::table
        .filter(price_data_::item_id.eq(&item.id))
        .filter(price_data_::timestamp.eq(timestamp))
        .count()
        .get_result::<i64>(conn)?
        != 0;

    if exists {
        diesel::update(price_data_::table)
            .filter(price_data_::item_id.eq(&item.id))
            .filter(price_data_::timestamp.eq(timestamp))
            .set((
                price_data_::base_price.eq(item.base_price),
                price_data_::avg_24h_price.eq(item.avg_24h_price),
            ))
            .execute(conn)?;
    } else {
        diesel::insert_into(price_data_::table)
            .values(&crate::db::models::NewPriceData {
                item_id: &item.id,
                timestamp,
                base_price: item.base_price,
                avg_24h_price: item.avg_24h_price,
            })
            .execute(conn)?;
    }

    Ok(())
}

fn upsert_trader_price_data(
    conn: &diesel::sqlite::SqliteConnection,
    item_id: &str,
    trader_price: &crate::fetch::models::TraderPrice,
    timestamp: i32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    upsert_trader(conn, &trader_price.trader)?;

    let exists = trader_price_data_::table
        .filter(trader_price_data_::item_id.eq(item_id))
        .filter(trader_price_data_::trader_id.eq(&trader_price.trader.id))
        .filter(trader_price_data_::timestamp.eq(timestamp))
        .count()
        .get_result::<i64>(conn)?
        != 0;

    if exists {
        diesel::update(trader_price_data_::table)
            .filter(trader_price_data_::item_id.eq(item_id))
            .filter(trader_price_data_::trader_id.eq(&trader_price.trader.id))
            .filter(trader_price_data_::timestamp.eq(timestamp))
            .set(trader_price_data_::price.eq(trader_price.price))
            .execute(conn)?;
    } else {
        diesel::insert_into(trader_price_data_::table)
            .values(&crate::db::models::NewTraderPriceData {
                item_id,
                trader_id: &trader_price.trader.id,
                timestamp,
                price: trader_price.price,
            })
            .execute(conn)?;
    }

    Ok(())
}

fn upsert_trader(
    conn: &diesel::sqlite::SqliteConnection,
    trader: &crate::fetch::models::Trader,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let exists = trader_::table
        .filter(trader_::id.eq(&trader.id))
        .count()
        .get_result::<i64>(conn)?
        != 0;

    if exists {
        diesel::update(trader_::table)
            .filter(trader_::id.eq(&trader.id))
            .set(trader_::name.eq(&trader.name))
            .execute(conn)?;
    } else {
        diesel::insert_into(trader_::table)
            .values(&crate::db::models::NewTrader {
                id: &trader.id,
                name: &trader.name,
            })
            .execute(conn)?;
    }

    Ok(())
}
