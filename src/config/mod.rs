/// Configuration management for the Fragarach framework
/// 
/// Handles loading and saving of application configuration, including:
/// - API keys management
/// - Environment variable integration
/// 
/// # Environment Variables
/// - `TRANSPOSE_API_KEY`: API key for Transpose service
/// - `URLSCAN_API_KEY`: API key for URLScan service
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Core configuration structure for the application
pub struct Config {
    transpose_api_key: Option<String>,
    urlscan_api_key: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            transpose_api_key: env::var("TRANSPOSE_API_KEY").ok(),
            urlscan_api_key: env::var("URLSCAN_API_KEY").ok(),
        }
    }

    pub fn transpose_api_key(&self) -> Option<String> {
        self.transpose_api_key.clone()
    }

    pub fn urlscan_api_key(&self) -> Option<String> {
        self.urlscan_api_key.clone()
    }

    pub fn set_transpose_api_key(&mut self, key: Option<String>) {
        self.transpose_api_key = key;
    }

    pub fn set_urlscan_api_key(&mut self, key: Option<String>) {
        self.urlscan_api_key = key;
    }
}