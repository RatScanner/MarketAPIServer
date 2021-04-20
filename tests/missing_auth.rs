mod util;

use market_api_server::init;
use warp::http::StatusCode;

#[tokio::test(flavor = "multi_thread")]
async fn get_all_resources() {
    let server = init(util::config()).await;
    let res = warp::test::request().path("/res").reply(&server).await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_resource() {
    let server = init(util::config()).await;
    let res = warp::test::request()
        .path("/res")
        .method("POST")
        .json(&serde_json::json!({
            "key": "  abc  ",
            "value": "  456 "
        }))
        .reply(&server)
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test(flavor = "multi_thread")]
async fn delete_resource() {
    let server = init(util::config()).await;
    let res = warp::test::request()
        .path("/res/xxxx")
        .method("DELETE")
        .reply(&server)
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test(flavor = "multi_thread")]
async fn get_all_files() {
    let server = init(util::config()).await;
    let res = warp::test::request().path("/file").reply(&server).await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test(flavor = "multi_thread")]
async fn upload_file() {
    let server = init(util::config()).await;
    let res = warp::test::request()
        .path("/file/xxx")
        .method("PUT")
        .body(&[42])
        .reply(&server)
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test(flavor = "multi_thread")]
async fn delete_file() {
    let server = init(util::config()).await;
    let res = warp::test::request()
        .path("/file/xxxx")
        .method("DELETE")
        .reply(&server)
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
