use super::{
    error::{self, ResultExt as _},
    models,
};
use crate::{db, state::StateHandle};

#[derive(rust_embed::RustEmbed)]
#[folder = "public"]
struct Asset;

pub async fn get_resource(req: tide::Request<StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let key = req.param::<String>("key").unwrap_or(String::from(""));
        let key = percent_encoding::percent_decode(key.as_bytes())
            .decode_utf8()
            .unwrap();
        let key = key.trim();

        let mut conn = db::get_connection().await.server_error()?;

        let resource = sqlx::query!("SELECT * FROM resource_ WHERE key = ?", key)
            .map(|record| models::Resource {
                key: record.key,
                value: record.value,
            })
            .fetch_optional(&mut conn)
            .await
            .server_error()?
            .ok_or(tide::http::StatusCode::NOT_FOUND)?;

        Ok(tide::Response::new(200)
            .body_json(&resource)
            .server_error()?)
    })
    .await
}

pub async fn get_all_resources(_: tide::Request<StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let mut conn = db::get_connection().await.server_error()?;

        let resources = sqlx::query!("SELECT * FROM resource_")
            .map(|record| models::Resource {
                key: record.key,
                value: record.value,
            })
            .fetch_all(&mut conn)
            .await
            .server_error()?;

        Ok(tide::Response::new(200)
            .body_json(&resources)
            .server_error()?)
    })
    .await
}

pub async fn post_resource(mut req: tide::Request<StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let mut conn = db::get_connection().await.server_error()?;

        let post_resource = req.body_json::<models::Resource>().await.client_error()?;
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

        Ok(tide::Response::new(201)
            .body_json(&post_resource)
            .server_error()?)
    })
    .await
}

pub async fn delete_resource(req: tide::Request<StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let key = req.param::<String>("key").unwrap_or(String::from(""));
        let key = percent_encoding::percent_decode(key.as_bytes())
            .decode_utf8()
            .unwrap();
        let key = key.trim();

        let mut conn = db::get_connection().await.server_error()?;

        sqlx::query!("DELETE FROM resource_ WHERE key = ?1", key)
            .execute(&mut conn)
            .await
            .server_error()?;

        Ok(tide::Response::new(204))
    })
    .await
}

pub async fn get_resource_editor(_: tide::Request<StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let data = Asset::get("resourceEditor.html").unwrap();

        Ok(tide::Response::new(200)
            .body_string(String::from(std::str::from_utf8(data.as_ref()).unwrap()))
            .set_header("Content-Type", "text/html"))
    })
    .await
}

pub async fn upload(req: tide::Request<StateHandle>) -> tide::Response {
    error::catch(|| async move {
        // File name
        let file_name = req.param::<String>("file").client_error()?;
        let file_name = percent_encoding::percent_decode(file_name.as_bytes())
            .decode_utf8()
            .unwrap();
        let file_name = file_name.trim();

        // File data
        let mut file_data = Vec::new();
        async_std::io::copy(req, &mut file_data)
            .await
            .server_error()?;

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

        Ok(tide::Response::new(200)
            .body_json(&serde_json::json!({}))
            .server_error()?)
    })
    .await
}

pub async fn get_file(req: tide::Request<StateHandle>) -> tide::Response {
    error::catch(|| async move {
        // File name
        let file_name = req.param::<String>("file").client_error()?;
        let file_name = percent_encoding::percent_decode(file_name.as_bytes())
            .decode_utf8()
            .unwrap();
        let file_name = file_name.trim();

        // Query
        let mut conn = db::get_connection().await.server_error()?;
        let file_data = sqlx::query_scalar!("SELECT data FROM file_ WHERE name = ?", file_name)
            .fetch_optional(&mut conn)
            .await
            .server_error()?
            .ok_or(tide::http::StatusCode::NOT_FOUND)?;
        let file_data_buffer = async_std::io::Cursor::new(file_data);

        Ok(tide::Response::with_reader(200, file_data_buffer)
            .set_mime(mime::APPLICATION_OCTET_STREAM))
    })
    .await
}

pub async fn get_all_endpoint(req: tide::Request<StateHandle>) -> tide::Response {
    #[derive(Debug, serde::Deserialize)]
    struct ReqQuery {
        lang: Option<String>,
    }

    error::catch(|| async move {
        let req_query = req.query::<ReqQuery>().client_error()?;

        let market_items = req
            .state()
            .get(req_query.lang.as_deref())
            .ok_or(tide::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        let market_items_buffer = async_std::io::Cursor::new(market_items);

        Ok(tide::Response::new(200)
            .body(market_items_buffer)
            .set_mime(mime::APPLICATION_JSON)
            .set_header("Content-Encoding", "gzip"))
    })
    .await
}
