use std::env;
use std::sync::Arc;

pub type ConfigHandle = Arc<Config>;

pub struct Config {
    pub database_url: String,
    pub auth_key: String,
    pub env: Environment,
    _private: (),
}

impl Config {
    pub fn new(database_url: String, auth_key: String, env: Environment) -> ConfigHandle {
        Arc::new(Self {
            database_url,
            auth_key,
            env,
            _private: (),
        })
    }

    pub fn from_env() -> ConfigHandle {
        Arc::new(Self {
            database_url: env::var("DATABASE_URL").expect("Could not find env DATABASE_URL"),
            auth_key: env::var("AUTH_KEY").expect("Could not find env AUTH_KEY"),
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
