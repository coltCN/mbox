use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub username: String,
    pub name: String,
    #[serde(alias = "type")]
    pub db_type: String,
}

impl Config {
    pub async fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        let config = Self::from_yaml(&content)?;
        Ok(config)
    }

    pub fn from_yaml(content: &str) -> Result<Self> {
        let config = serde_yaml::from_str(content)?;

        Ok(config)
    }
}
