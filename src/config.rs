// MUTABLE CONFIG

/// Struct representing the application configuration file.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

/// Struct representing the database configuration.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

/// Struct representing the authentication configuration.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AuthConfig {
    pub url: String,
    pub root: RootConfig,
}

/// Struct representing the root authentication configuration.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct RootConfig {
    pub password: String,
}

impl Config {
    /// Reads the configuration file from the given path.
    pub fn read(path: String) -> Result<Self, crate::errors::config::ConfigReadError> {
        use serde_json::from_str;
        use std::fs::read_to_string;

        let content = read_to_string(path)?;
        let parsed = from_str::<Self>(&content)?;

        Ok(parsed)
    }
}

// IMMUTABLE CONFIG - accessible only by the application
/// The internal name of the service used as an ID while querying authios and other external services.
pub const SERVICE_NAME: &str = "taskios";
