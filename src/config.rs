// MUTABLE CONFIG
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AuthConfig {
    pub url: String,
    pub root: RootConfig,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct RootConfig {
    pub password: String,
}

impl Config {
    pub fn read(path: String) -> Result<Self, crate::errors::config::ConfigReadError> {
        use serde_json::from_str;
        use std::fs::read_to_string;

        let content = read_to_string(path)?;
        let parsed = from_str::<Self>(&content)?;

        Ok(parsed)
    }
}

// IMMUTABLE CONFIG - accessible only by the application
pub const AUTHIOS_SERVICE_NAME: &str = "taskios";
