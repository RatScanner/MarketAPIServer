pub mod models;

#[derive(thiserror::Error, Debug)]
pub enum FetchError {
    #[error("Surf error {0}")]
    Surf(#[from] surf::Exception),

    #[error("Io error {0}")]
    Io(#[from] std::io::Error),

    #[error("Status error")]
    StatusError,
}

pub async fn fetch(lang: Option<&str>) -> Result<Vec<models::MarketItem>, FetchError> {
    let api_key =
        std::env::var("TARKOV_MARKET_API_KEY").expect("Could not find env TARKOV_MARKET_API_KEY");

    let base_uri = "https://tarkov-market.com/api/v1/items/all";
    let uri = match lang {
        Some(lang) => format!("{}?lang={}", base_uri, lang),
        None => base_uri.to_string(),
    };

    let mut response = surf::get(uri).set_header("x-api-key", api_key).await?;

    if !response.status().is_success() {
        return Err(FetchError::StatusError);
    }

    let market_items = response.body_json::<Vec<models::MarketItem>>().await?;

    Ok(market_items)
}
