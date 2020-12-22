use super::state::StateHandle;

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
    let languages = ["en", "ru", "de", "fr", "es", "cn"];
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
    lang: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use diesel::prelude::*;

    // Fetch
    let market_items = crate::fetch::fetch(Some(lang)).await?;

    // Write to db
    let conn = super::get_db_connection()?;

    conn.transaction::<_, Box<dyn std::error::Error + Send + Sync + 'static>, _>(|| {
        for market_item in market_items {
            upsert_market_item(&conn, &market_item)?;
            upsert_market_item_name(&conn, &market_item, lang)?;
            upsert_price_data(&conn, &market_item)?;
        }
        Ok(())
    })?;

    Ok(())
}

fn upsert_market_item(
    conn: &diesel::sqlite::SqliteConnection,
    market_item: &crate::fetch::models::MarketItem,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use crate::schema::*;
    use diesel::prelude::*;

    let exists = market_item::table
        .filter(market_item::uid.eq(&market_item.uid))
        .count()
        .get_result::<i64>(conn)?
        != 0;

    if exists {
        diesel::update(market_item::table)
            .filter(market_item::uid.eq(&market_item.uid))
            .set((
                market_item::slots.eq(market_item.slots),
                market_item::wiki_link.eq(&market_item.wiki_link),
                market_item::img_link.eq(&market_item.img_link),
            ))
            .execute(conn)?;
    } else {
        diesel::insert_into(market_item::table)
            .values(&crate::db::models::NewMarketItem {
                uid: &market_item.uid,
                slots: market_item.slots,
                wiki_link: &market_item.wiki_link,
                img_link: &market_item.img_link,
            })
            .execute(conn)?;
    }

    Ok(())
}

fn upsert_market_item_name(
    conn: &diesel::sqlite::SqliteConnection,
    market_item: &crate::fetch::models::MarketItem,
    lang: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use crate::schema::*;
    use diesel::prelude::*;

    let exists = market_item_name::table
        .filter(market_item_name::uid.eq(&market_item.uid))
        .filter(market_item_name::lang.eq(lang))
        .count()
        .get_result::<i64>(conn)?
        != 0;

    if exists {
        diesel::update(market_item_name::table)
            .filter(market_item_name::uid.eq(&market_item.uid))
            .filter(market_item_name::lang.eq(lang))
            .set((
                market_item_name::name.eq(&market_item.name),
                market_item_name::short_name.eq(&market_item.short_name),
            ))
            .execute(conn)?;
    } else {
        diesel::insert_into(market_item_name::table)
            .values(&crate::db::models::NewMarketItemName {
                uid: &market_item.uid,
                lang,
                name: &market_item.name,
                short_name: &market_item.short_name,
            })
            .execute(conn)?;
    }

    Ok(())
}

fn upsert_price_data(
    conn: &diesel::sqlite::SqliteConnection,
    market_item: &crate::fetch::models::MarketItem,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use crate::schema::*;
    use diesel::prelude::*;

    let exists = price_data::table
        .filter(price_data::uid.eq(&market_item.uid))
        .filter(price_data::timestamp.eq(market_item.timestamp))
        .count()
        .get_result::<i64>(conn)?
        != 0;

    if exists {
        diesel::update(price_data::table)
            .filter(price_data::uid.eq(&market_item.uid))
            .filter(price_data::timestamp.eq(market_item.timestamp))
            .set((
                price_data::price.eq(market_item.price),
                price_data::avg_24h_price.eq(market_item.avg_24h_price),
                price_data::avg_7d_price.eq(market_item.avg_7d_price),
                price_data::trader_name.eq(&market_item.trader_name),
                price_data::trader_price.eq(market_item.trader_price),
                price_data::trader_currency.eq(&market_item.trader_currency),
            ))
            .execute(conn)?;
    } else {
        diesel::insert_into(price_data::table)
            .values(&crate::db::models::NewPriceData {
                uid: &market_item.uid,
                timestamp: market_item.timestamp,
                price: market_item.price,
                avg_24h_price: market_item.avg_24h_price,
                avg_7d_price: market_item.avg_7d_price,
                trader_name: &market_item.trader_name,
                trader_price: market_item.trader_price,
                trader_currency: &market_item.trader_currency,
            })
            .execute(conn)?;
    }

    Ok(())
}
