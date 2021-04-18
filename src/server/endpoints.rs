use super::{
    error::{Error, ResultExt as _},
    models,
    util::PercentDecoded,
};
use crate::{db, state::StateHandle};
use warp::Reply;

type Result<T> = std::result::Result<T, warp::Rejection>;

#[derive(rust_embed::RustEmbed)]
#[folder = "public"]
struct Asset;

pub async fn get_resource_editor() -> Result<impl Reply> {
    let data = Asset::get("resourceEditor.html").unwrap();

    Ok(warp::reply::html(data))
}

pub async fn get_resource(key: PercentDecoded) -> Result<impl Reply> {
    let key = key.as_ref().trim();
    let mut conn = db::get_connection().await.server_error()?;

    let resource = sqlx::query!("SELECT * FROM resource_ WHERE key = ?", key)
        .map(|record| models::Resource {
            key: record.key,
            value: record.value,
        })
        .fetch_optional(&mut conn)
        .await
        .server_error()?
        .ok_or(Error::not_found())?;

    Ok(warp::reply::json(&resource))
}

pub async fn get_all_resources() -> Result<impl Reply> {
    let mut conn = db::get_connection().await.server_error()?;

    let resources = sqlx::query!("SELECT * FROM resource_")
        .map(|record| models::Resource {
            key: record.key,
            value: record.value,
        })
        .fetch_all(&mut conn)
        .await
        .server_error()?;

    Ok(warp::reply::json(&resources))
}

pub async fn post_resource(post_resource: models::Resource) -> Result<impl Reply> {
    let mut conn = db::get_connection().await.server_error()?;

    let post_resource = models::Resource {
        key: post_resource.key.trim().to_string(),
        value: post_resource.value,
    };

    sqlx::query!(
        r#"
            INSERT INTO resource_ (key, value)
            VALUES(?1, ?2)
            ON CONFLICT(key)
            DO UPDATE SET value = ?2
            "#,
        post_resource.key,
        post_resource.value,
    )
    .execute(&mut conn)
    .await
    .server_error()?;

    Ok(warp::reply::json(&post_resource))
}

pub async fn delete_resource(key: PercentDecoded) -> Result<impl Reply> {
    let key = key.as_ref().trim();
    let mut conn = db::get_connection().await.server_error()?;

    sqlx::query!("DELETE FROM resource_ WHERE key = ?1", key)
        .execute(&mut conn)
        .await
        .server_error()?;

    Ok(warp::reply::json(&serde_json::json!({})))
}

pub async fn upload(file_name: PercentDecoded, body: bytes::Bytes) -> Result<impl Reply> {
    // File
    let file_name = file_name.as_ref().trim();
    let file_data = &*body;

    // Insert
    let mut conn = db::get_connection().await.server_error()?;
    sqlx::query!(
        r#"
            INSERT INTO file_ (name, data)
            VALUES(?1, ?2)
            ON CONFLICT(name)
            DO UPDATE SET data = ?2
            "#,
        file_name,
        file_data,
    )
    .execute(&mut conn)
    .await
    .server_error()?;

    Ok(warp::reply::json(&serde_json::json!({})))
}

pub async fn get_file(file_name: PercentDecoded) -> Result<impl Reply> {
    // File
    let file_name = file_name.as_ref().trim();

    // Query
    let mut conn = db::get_connection().await.server_error()?;
    let file_data = sqlx::query_scalar!("SELECT data FROM file_ WHERE name = ?", file_name)
        .fetch_optional(&mut conn)
        .await
        .server_error()?
        .ok_or(Error::not_found())?;

    Ok(warp::http::Response::builder()
        .header(
            warp::http::header::CONTENT_TYPE,
            warp::http::HeaderValue::from_static("application/octet-stream"),
        )
        .body(file_data)
        .unwrap())
}

#[derive(Debug, serde::Deserialize)]
pub struct GetAllItemsReqQuery {
    lang: Option<String>,
}

pub async fn get_all_items(query: GetAllItemsReqQuery, state: StateHandle) -> Result<impl Reply> {
    let market_items = state
        .get(query.lang.as_deref())
        .ok_or(Error::server_error())?;

    Ok(warp::http::Response::builder()
        .header(
            warp::http::header::CONTENT_TYPE,
            warp::http::HeaderValue::from_static("application/json"),
        )
        .header(
            warp::http::header::CONTENT_ENCODING,
            warp::http::HeaderValue::from_static("gzip"),
        )
        .body(market_items)
        .unwrap())
}
