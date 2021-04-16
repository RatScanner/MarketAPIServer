mod error;
mod models;
mod service;
mod state;

use error::ResultExt as _;
use sqlx::Connection;

#[derive(rust_embed::RustEmbed)]
#[folder = "public"]
struct Asset;

pub async fn start() {
    // Run migrations
    run_migrations().await.unwrap();

    // Init state
    let state = state::StateHandle::new();

    // Start service
    service::start(state.clone());

    // Start server
    let mut app = tide::with_state(state.clone());

    app.middleware(not_found);

    let mut authed_router = tide::with_state(state);
    authed_router.middleware(authenticate);

    app.at("/res/:key").get(get_resource);
    authed_router.at("").get(get_all_resources);
    authed_router.at("").post(post_resource);
    authed_router.at("/:key").delete(delete_resource);

    app.at("/").get(|_req| async { "Market API Server" });
    app.at("/resEditor").get(get_resource_editor);
    app.at("/res").nest(authed_router);

    app.at("/all").get(get_all_endpoint);

    println!("Server started!");

    app.listen("0.0.0.0:8080").await.unwrap();
}

async fn get_db_connection() -> sqlx::Result<sqlx::SqliteConnection> {
    let database_url = std::env::var("DATABASE_URL").expect("Could not find env DATABASE_URL");
    Ok(sqlx::SqliteConnection::connect(&database_url).await?)
}

async fn run_migrations() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut conn = get_db_connection().await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;
    Ok(())
}

async fn get_resource(req: tide::Request<state::StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let key = req.param::<String>("key").unwrap_or(String::from(""));
        let key = percent_encoding::percent_decode(key.as_bytes())
            .decode_utf8()
            .unwrap();
        let key = key.trim();

        let mut conn = get_db_connection().await.server_error()?;

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

async fn get_all_resources(_: tide::Request<state::StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let mut conn = get_db_connection().await.server_error()?;

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

async fn post_resource(mut req: tide::Request<state::StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let mut conn = get_db_connection().await.server_error()?;

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

async fn delete_resource(req: tide::Request<state::StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let key = req.param::<String>("key").unwrap_or(String::from(""));
        let key = percent_encoding::percent_decode(key.as_bytes())
            .decode_utf8()
            .unwrap();
        let key = key.trim();

        let mut conn = get_db_connection().await.server_error()?;

        sqlx::query!("DELETE FROM resource_ WHERE key = ?1", key)
            .execute(&mut conn)
            .await
            .server_error()?;

        Ok(tide::Response::new(204))
    })
    .await
}

async fn get_resource_editor(_: tide::Request<state::StateHandle>) -> tide::Response {
    error::catch(|| async move {
        let data = Asset::get("resourceEditor.html").unwrap();

        Ok(tide::Response::new(200)
            .body_string(String::from(std::str::from_utf8(data.as_ref()).unwrap()))
            .set_header("Content-Type", "text/html"))
    })
    .await
}

async fn get_all_endpoint(req: tide::Request<state::StateHandle>) -> tide::Response {
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

type BoxFuture<'a, T> =
    std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = T> + Send + 'a>>;

// Custom 404 repsonse
// TODO replace middleware with something better
fn not_found(
    req: tide::Request<state::StateHandle>,
    next: tide::Next<'_, state::StateHandle>,
) -> BoxFuture<'_, tide::Response> {
    use tide::http::StatusCode;

    Box::pin(async move {
        let response = next.run(req).await;

        if response.status() == StatusCode::NOT_FOUND {
            self::error::error_response(StatusCode::NOT_FOUND, None)
        } else {
            response
        }
    })
}

fn authenticate(
    req: tide::Request<state::StateHandle>,
    next: tide::Next<'_, state::StateHandle>,
) -> BoxFuture<'_, tide::Response> {
    use tide::http::StatusCode;

    Box::pin(async move {
        let auth_key = std::env::var("AUTH_KEY").expect("Could not find env AUTH_KEY");

        let auth_key_header = req.header("x-auth-key");

        match auth_key_header {
            Some(key) if key == auth_key => next.run(req).await,
            _ => self::error::error_response(StatusCode::UNAUTHORIZED, None),
        }
    })
}
