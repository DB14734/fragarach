/// Configuration management for the Fragarach framework
/// 
/// Handles loading and saving of application configuration, including:
/// - Database preferences (SQLite/PostgreSQL)
/// - API keys management
/// - Environment variable integration
/// 
/// # Environment Variables
/// - `SAVE_AS_SQLITE`: Enable SQLite storage (default: true)
/// - `SAVE_AS_POSTGRES`: Enable PostgreSQL storage (default: false)
/// - `TRANSPOSE_API_KEY`: API key for Transpose service
/// - `POSTGRES_URL`: PostgreSQL connection string
/// - `URLSCAN_API_KEY`: API key for URLScan service
use dotenv::dotenv;
use std::env;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Core configuration structure for the application
pub struct Config {
    pub save_as_sqlite: bool,
    pub save_as_postgres: bool,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        if let Ok(config) = Self::load() {
            config
        } else {
            Config {
                save_as_sqlite: env::var("SAVE_AS_SQLITE").unwrap_or_else(|_| "true".to_string()).to_lowercase() == "true",
                save_as_postgres: env::var("SAVE_AS_POSTGRES").unwrap_or_else(|_| "false".to_string()).to_lowercase() == "true",
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_json = serde_json::to_string(self)?;
        fs::write("config.json", config_json)?;
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_json = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&config_json)?;
        Ok(config)
    }

    pub fn transpose_api_key(&self) -> Option<String> {
        env::var("TRANSPOSE_API_KEY").ok()
    }

    pub fn postgres_url(&self) -> Option<String> {
        env::var("POSTGRES_URL").ok()
    }

    pub fn urlscan_api_key(&self) -> Option<String> {
        env::var("URLSCAN_API_KEY").ok()
    }
}