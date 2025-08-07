use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub api_version: String,
}

impl DatabaseSettings {
    pub fn get_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Detect the running environment
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            // Application defaults
            .set_default("application.host", "127.0.0.1")?
            .set_default("application.port", 8080)?
            .set_default("application.environment", "development")?
            .set_default("application.api_version", "v1")?
            // Database defaults
            .set_default("database.host", "localhost")?
            .set_default("database.port", 5432)?
            .set_default("database.name", "rust_api_db")?
            .set_default("database.username", "postgres")?
            .set_default("database.password", "password")?
            .set_default("database.max_connections", 10)?
            .set_default("database.min_connections", 5)?;

        // Add environment-specific configuration file if it exists
        let config_file = format!("config/{}.toml", environment);
        if std::path::Path::new(&config_file).exists() {
            builder = builder.add_source(File::with_name(&config_file));
        }

        // Add environment variables with APP_ prefix
        builder = builder.add_source(
            Environment::with_prefix("APP")
                .separator("_")
                .prefix_separator("_"),
        );

        // Also check for common DB_ prefixed variables
        if let Ok(db_host) = env::var("DB_HOST") {
            builder = builder.set_override("database.host", db_host)?;
        }
        if let Ok(db_port) = env::var("DB_PORT") {
            builder = builder.set_override("database.port", db_port)?;
        }
        if let Ok(db_name) = env::var("DB_NAME") {
            builder = builder.set_override("database.name", db_name)?;
        }
        if let Ok(db_user) = env::var("DB_USER") {
            builder = builder.set_override("database.username", db_user)?;
        }
        if let Ok(db_password) = env::var("DB_PASSWORD") {
            builder = builder.set_override("database.password", db_password)?;
        }

        builder.build()?.try_deserialize()
    }

    pub fn get_bind_address(&self) -> String {
        format!("{}:{}", self.application.host, self.application.port)
    }
}
