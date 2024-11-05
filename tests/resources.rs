mod util;

use market_api_server::{init, server::models};
use std::collections::HashSet;
use util::ResponseExt;

#[tokio::test(flavor = "multi_thread")]
async fn post_and_get() {
    let server = init(util::config()).await;

    warp::test::request()
        .path("/res")
        .method("POST")
        .header("x-auth-key", "1234")
        .json(&serde_json::json!({
            "key": "  abc  ",
            "value": "  456 "
        }))
        .reply(&server)
        .await;

    let res = warp::test::request()
        .path("/res/abc")
        .reply(&server)
        .await
        .body_json::<models::Resource>();

    assert_eq!(
        res,
        models::Resource {
            key: "abc".to_string(),
            value: "  456 ".to_string()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn post_and_get_all() {
    let server = init(util::config()).await;

    warp::test::request()
        .path("/res")
        .method("POST")
        .header("x-auth-key", "1234")
        .json(&serde_json::json!({
            "key": "  abc  ",
            "value": "  456 "
        }))
        .reply(&server)
        .await;

    warp::test::request()
        .path("/res")
        .method("POST")
        .header("x-auth-key", "1234")
        .json(&serde_json::json!({
            "key": "cde",
            "value": "Hello World"
        }))
        .reply(&server)
        .await;

    warp::test::request()
        .path("/res")
        .method("POST")
        .header("x-auth-key", "1234")
        .json(&serde_json::json!({
            "key": "abc",
            "value": "0000"
        }))
        .reply(&server)
        .await;

    let res = warp::test::request()
        .path("/res")
        .header("x-auth-key", "1234")
        .reply(&server)
        .await
        .body_json::<HashSet<models::Resource>>();

    assert_eq!(
        res,
        vec![
            models::Resource {
                key: "abc".to_string(),
                value: "0000".to_string()
            },
            models::Resource {
                key: "cde".to_string(),
                value: "Hello World".to_string()
            }
        ]
        .into_iter()
        .collect::<HashSet<_>>()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn post_and_delete_and_get_all() {
    let server = init(util::config()).await;

    warp::test::request()
        .path("/res")
        .method("POST")
        .header("x-auth-key", "1234")
        .json(&serde_json::json!({
            "key": "  abc     ",
            "value": "  456 "
        }))
        .reply(&server)
        .await;

    warp::test::request()
        .path("/res")
        .method("POST")
        .header("x-auth-key", "1234")
        .json(&serde_json::json!({
            "key": "cde",
            "value": "Hello World"
        }))
        .reply(&server)
        .await;

    warp::test::request()
        .path("/res/%20abc%20")
        .method("DELETE")
        .header("x-auth-key", "1234")
        .reply(&server)
        .await;

    let res = warp::test::request()
        .path("/res")
        .header("x-auth-key", "1234")
        .reply(&server)
        .await
        .body_json::<HashSet<models::Resource>>();

    assert_eq!(
        res,
        vec![models::Resource {
            key: "cde".to_string(),
            value: "Hello World".to_string()
        }]
        .into_iter()
        .collect::<HashSet<_>>()
    );
}
