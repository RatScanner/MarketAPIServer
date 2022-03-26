use serde_json::json;

pub mod models;

#[derive(thiserror::Error, Debug)]
pub enum FetchError {
    #[error("Reqwest error {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Graphql error")]
    Graphql,
}

pub async fn fetch() -> Result<Vec<models::Item>, FetchError> {
    let uri = "https://api.tarkov.dev/graphql";
    let body = json!({
        "query": "{itemsByType(type: any) {id,basePrice,updated,iconLink,wikiLink,imageLink,avg24hPrice,traderPrices { price,trader {id,name}}}}",
    });

    let response = reqwest::Client::new()
        .post(uri)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    match response.json::<models::Response>().await? {
        models::Response::Data { items_by_type } => Ok(items_by_type),
        models::Response::Error {} => Err(FetchError::Graphql),
    }
}
