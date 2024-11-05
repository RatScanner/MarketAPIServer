use super::{
    error::{Error, ResultExt as _},
    models,
    util::PercentDecoded,
};
use crate::{db::Db, ConfigHandle};
use reqwest::StatusCode;
use serde_json::json;
use warp::Reply;

type Result<T> = std::result::Result<T, warp::Rejection>;

#[derive(rust_embed::RustEmbed)]
#[folder = "public"]
struct Asset;

pub async fn get_resource_editor() -> Result<impl Reply> {
    let data = Asset::get("resourceEditor.html").unwrap();

    Ok(warp::reply::html(data))
}

pub async fn get_resource(key: PercentDecoded, db: Db) -> Result<impl Reply> {
    let key = key.as_ref().trim();
    let mut conn = db.conn().await.server_error()?;

    let resource = sqlx::query!("SELECT * FROM resource_ WHERE key = $1", key)
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

pub async fn get_all_resources(db: Db) -> Result<impl Reply> {
    let mut conn = db.conn().await.server_error()?;

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

pub async fn post_resource(post_resource: models::Resource, db: Db) -> Result<impl Reply> {
    let mut conn = db.conn().await.server_error()?;

    let post_resource = models::Resource {
        key: post_resource.key.trim().to_string(),
        value: post_resource.value,
    };

    sqlx::query!(
        r#"
            INSERT INTO resource_ (key, value)
            VALUES($1, $2)
            ON CONFLICT(key)
            DO UPDATE SET value = $2
            "#,
        post_resource.key,
        post_resource.value,
    )
    .execute(&mut conn)
    .await
    .server_error()?;

    Ok(warp::reply::json(&post_resource))
}

pub async fn delete_resource(key: PercentDecoded, db: Db) -> Result<impl Reply> {
    let key = key.as_ref().trim();
    let mut conn = db.conn().await.server_error()?;

    sqlx::query!("DELETE FROM resource_ WHERE key = $1", key)
        .execute(&mut conn)
        .await
        .server_error()?;

    Ok(warp::reply::json(&serde_json::json!({})))
}

pub async fn upload_file(
    file_name: PercentDecoded,
    body: bytes::Bytes,
    db: Db,
) -> Result<impl Reply> {
    // File
    let file_name = file_name.as_ref().trim();
    let file_data = &*body;

    // Insert
    let mut conn = db.conn().await.server_error()?;
    sqlx::query!(
        r#"
            INSERT INTO file_ (name, data)
            VALUES($1, $2)
            ON CONFLICT(name)
            DO UPDATE SET data = $2
            "#,
        file_name,
        file_data,
    )
    .execute(&mut conn)
    .await
    .server_error()?;

    Ok(warp::reply::json(&serde_json::json!({})))
}

pub async fn get_file(file_name: PercentDecoded, db: Db) -> Result<impl Reply> {
    // File
    let file_name = file_name.as_ref().trim();

    // Query
    let mut conn = db.conn().await.server_error()?;
    let file_data = sqlx::query_scalar!("SELECT data FROM file_ WHERE name = $1", file_name)
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

pub async fn get_all_files(db: Db) -> Result<impl Reply> {
    let mut conn = db.conn().await.server_error()?;

    let files = sqlx::query_scalar!("SELECT name FROM file_")
        .fetch_all(&mut conn)
        .await
        .server_error()?;

    Ok(warp::reply::json(&files))
}

pub async fn delete_file(file_name: PercentDecoded, db: Db) -> Result<impl Reply> {
    let file_name = file_name.as_ref().trim();
    let mut conn = db.conn().await.server_error()?;

    sqlx::query!("DELETE FROM file_ WHERE name = $1", file_name)
        .execute(&mut conn)
        .await
        .server_error()?;

    Ok(warp::reply::json(&serde_json::json!({})))
}

pub async fn post_oauth_refresh(
    conf: ConfigHandle,
    data: models::OAuthRefreshData,
) -> Result<impl Reply> {
    if data.client_id != conf.oauth_client_id_discord {
        return Err(Error::from(StatusCode::BAD_REQUEST).into());
    }

    let uri = &conf.oauth_client_token_endpoint_discord;
    let body = json!({
        "grant_type": "refresh_token",
        "client_id": conf.oauth_client_id_discord,
        "client_secret": conf.oauth_client_secret_discord,
        "refresh_token": data.refresh_token,
    });

    let response = reqwest::Client::new()
        .post(uri)
        .form(&body)
        .send()
        .await
        .server_error()?;

    let data = response
        .error_for_status()
        .server_error()?
        .json::<models::AccessTokenResponse>()
        .await
        .server_error()?;

    Ok(warp::reply::json(&data))
}
