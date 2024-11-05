#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OAuthRefreshData {
    #[serde(rename = "client_id")]
    pub client_id: String,

    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,

    #[serde(rename = "token_type")]
    pub token_type: String,

    #[serde(rename = "expires_in")]
    pub expires_in: i32,

    #[serde(rename = "refresh_token")]
    pub refresh_token: String,

    #[serde(rename = "scope")]
    pub scope: String,
}
