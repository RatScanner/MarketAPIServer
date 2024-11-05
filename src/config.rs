use std::env;
use std::sync::Arc;

pub type ConfigHandle = Arc<Config>;

pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub auth_key: String,
    pub oauth_client_id_discord: String,
    pub oauth_client_secret_discord: String,
    pub oauth_client_token_endpoint_discord: String,
    pub env: Environment,
    _private: (),
}

impl Config {
    pub fn new(
        port: u16,
        database_url: impl Into<String>,
        auth_key: impl Into<String>,
        oauth_client_id_discord: impl Into<String>,
        oauth_client_secret_discord: impl Into<String>,
        oauth_client_token_endpoint_discord: impl Into<String>,
        env: Environment,
    ) -> ConfigHandle {
        Arc::new(Self {
            port,
            database_url: database_url.into(),
            auth_key: auth_key.into(),
            oauth_client_id_discord: oauth_client_id_discord.into(),
            oauth_client_secret_discord: oauth_client_secret_discord.into(),
            oauth_client_token_endpoint_discord: oauth_client_token_endpoint_discord.into(),
            env,
            _private: (),
        })
    }

    pub fn from_env() -> ConfigHandle {
        Arc::new(Self {
            port: env::var("PORT")
                .expect("Could not find env PORT")
                .parse()
                .expect("Invalid value for PORT"),
            database_url: env::var("DATABASE_URL").expect("Could not find env DATABASE_URL"),
            auth_key: env::var("AUTH_KEY").expect("Could not find env AUTH_KEY"),
            oauth_client_id_discord: env::var("OAUTH_CLIENT_ID_DISCORD")
                .expect("Could not find env OAUTH_CLIENT_ID_DISCORD"),
            oauth_client_secret_discord: env::var("OAUTH_CLIENT_SECRET_DISCORD")
                .expect("Could not find env OAUTH_CLIENT_SECRET_DISCORD"),
            oauth_client_token_endpoint_discord: env::var("OAUTH_CLIENT_TOKEN_ENDPOINT_DISCORD")
                .expect("Could not find env OAUTH_CLIENT_TOKEN_ENDPOINT_DISCORD"),
            env: match env::var("ENVIRONMENT")
                .expect("Could not find env ENVIRONMENT")
                .as_str()
            {
                "Production" => Environment::Production,
                "Development" => Environment::Development,
                "Test" => Environment::Test,
                env => panic!("Invalid value for ENVIRONMENT: {}", env),
            },
            _private: (),
        })
    }
}

#[derive(Debug)]
pub enum Environment {
    Production,
    Development,
    Test,
}
