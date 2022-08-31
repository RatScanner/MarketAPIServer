use super::state::{State, StateHandle, LANGUAGES};
use crate::db::Db;
use sqlx::{Connection, Postgres, Transaction};

pub fn start(state: StateHandle, db: Db) {
    tokio::spawn(async move {
        run(&state, &db).await;
    });
}

async fn run(state: &State, db: &Db) {
    let mut languages_cycle = LANGUAGES.iter().cycle();

    loop {
        // Fetch and update
        let res = fetch_and_update(languages_cycle.next().unwrap(), db).await;

        match res {
            Ok(_) => {
                if let Err(e) = state.update_from_db(db).await {
                    log::error!("failed to update from db: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(60 * 10)).await;
            }
            Err(e) => {
                log::error!("failed to fetch and update: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        }
    }
}

async fn fetch_and_update(
    _lang: &str,
    db: &Db,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Fetch
    let timestamp_fallback = chrono::Utc::now().timestamp();
    let items = crate::fetch::fetch().await?;
    log::info!("Fetched successfully");

    // Write to db
    let mut conn = db.conn().await?;
    conn.transaction::<_, _, Box<dyn std::error::Error + Send + Sync + 'static>>(move |conn| {
        Box::pin(async move {
            for item in items {
                // Calc timestamp
                let timestamp = match &item.updated {
                    Some(_) => {
                        // TODO: Parse timestamp from item.updated
                        timestamp_fallback
                    }
                    None => timestamp_fallback,
                };

                // Upsert ...
                upsert_item(conn, &item).await?;
                upsert_price_data(conn, &item, timestamp).await?;
                for trader_price in &item.trader_prices {
                    upsert_trader_price_data(conn, &item.id, trader_price).await?;
                }
            }

            Ok(())
        })
    })
    .await?;
    log::info!("Updated db successfully");

    Ok(())
}

async fn upsert_item(
    conn: &mut Transaction<'_, Postgres>,
    item: &crate::fetch::models::Item,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    sqlx::query!(
        r#"
        INSERT INTO item_ (id, icon_link, wiki_link, image_link)
        VALUES($1, $2, $3, $4)
        ON CONFLICT(id) 
        DO UPDATE SET
            icon_link = $2,
            wiki_link = $3,
            image_link = $4
        "#,
        item.id,
        item.icon_link,
        item.wiki_link,
        item.image_link,
    )
    .execute(conn)
    .await?;

    Ok(())
}

async fn upsert_price_data(
    conn: &mut Transaction<'_, Postgres>,
    item: &crate::fetch::models::Item,
    timestamp: i64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Do not upsert if item already exists with same base_price and avg_24h_price
    if sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT * FROM price_data_
            WHERE item_id = $1 AND base_price = $2 AND avg_24h_price = $3
        )
        "#,
        item.id,
        item.base_price,
        item.avg_24h_price,
    )
    .fetch_one(&mut *conn)
    .await?
    .unwrap_or(false)
    {
        return Ok(());
    }

    let avg_24h_price = match item.avg_24h_price {
        Some(price) => price,
        None => 0,
    };

    // Upsert price_data
    sqlx::query!(
        r#"
        INSERT INTO price_data_ (item_id, timestamp, base_price, avg_24h_price)
        VALUES($1, $2, $3, $4)
        ON CONFLICT(item_id, timestamp) 
        DO UPDATE SET
            base_price = $3,
            avg_24h_price = $4
        "#,
        item.id,
        timestamp,
        item.base_price,
        avg_24h_price,
    )
    .execute(conn)
    .await?;

    Ok(())
}

async fn upsert_trader_price_data(
    conn: &mut Transaction<'_, Postgres>,
    item_id: &str,
    trader_price: &crate::fetch::models::TraderPrice,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    upsert_trader(conn, &trader_price.trader).await?;

    sqlx::query!(
        r#"
        INSERT INTO trader_price_data_ (item_id, trader_id, price)
        VALUES($1, $2, $3)
        ON CONFLICT(item_id, trader_id) 
        DO UPDATE SET price = $3
        "#,
        item_id,
        trader_price.trader.id,
        trader_price.price,
    )
    .execute(conn)
    .await?;

    Ok(())
}

async fn upsert_trader(
    conn: &mut Transaction<'_, Postgres>,
    trader: &crate::fetch::models::Trader,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    sqlx::query!(
        r#"
        INSERT INTO trader_ (id, name)
        VALUES($1, $2)
        ON CONFLICT(id) 
        DO UPDATE SET name = $2
        "#,
        trader.id,
        trader.name,
    )
    .execute(conn)
    .await?;

    Ok(())
}
