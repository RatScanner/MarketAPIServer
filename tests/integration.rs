mod util;

use market_api_server::{models, start};

#[tokio::test(flavor = "multi_thread")]
#[serial_test::serial]
async fn res() {
    let server = start(util::config(), ([0, 0, 0, 0], 8081));
    let client = async {
        reqwest::Client::new()
            .post("http://localhost:8081/res")
            .header("x-auth-key", "1234")
            .json(&serde_json::json!({
                "key": "abc",
                "value": "123"
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        let res = reqwest::Client::new()
            .get("http://localhost:8081/res/abc")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json::<models::Resource>()
            .await
            .unwrap();

        assert_eq!(
            res,
            models::Resource {
                key: "abc".to_string(),
                value: "123".to_string()
            }
        );
    };

    tokio::select! {
        _ = server => {}
        _ = client => {}
    };
}
