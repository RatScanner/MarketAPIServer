use serde_json::json;

pub mod models;

#[derive(thiserror::Error, Debug)]
pub enum FetchError {
    #[error("Surf error {0}")]
    Surf(#[from] surf::Exception),

    #[error("Io error {0}")]
    Io(#[from] std::io::Error),

    #[error("Status error {0}")]
    StatusError(surf::http::StatusCode),

    #[error("Graphql error")]
    Graphql,
}

pub async fn fetch() -> Result<Vec<models::Item>, FetchError> {
    let uri = "https://tarkov-tools.com/graphql";
    let body = json!({
        "query": "{itemsByType(type: any) {id,basePrice,updated,iconLink,wikiLink,imageLink,avg24hPrice,traderPrices { price,trader {id,name}}}}",
    });

    let mut response = surf::post(uri).body_json(&body).unwrap().await?;

    if !response.status().is_success() {
        return Err(FetchError::StatusError(response.status()));
    }

    match response.body_json::<models::Response>().await? {
        models::Response::Data { items_by_type } => Ok(items_by_type),
        models::Response::Error {} => Err(FetchError::Graphql),
    }
}
