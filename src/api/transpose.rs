/// Transpose API integration for Ethereum blockchain data retrieval
/// 
/// Provides functionality to:
/// - Query Ethereum account details
/// - Retrieve transaction history
/// - Handle rate limiting and pagination
/// 
/// # Rate Limiting
/// Implements a 1-second delay between requests to comply with API limits
/// 
/// # Response Size
/// Monitors response size and implements a 1MB limit safeguard
use crate::config::Config;
use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Loads SQL query templates from files
pub async fn load_sql_query(filename: &str) -> String {
    let filepath = format!("src/sql/{}", filename);
    fs::read_to_string(&filepath).expect(&format!("Unable to read file: {}", filepath))
}

pub async fn query_transpose(config: &Config, sql_query: &str, params: &[(&str, &str)]) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.transpose.io/sql";

    let mut query = sql_query.to_string();
    for (key, value) in params {
        query = query.replace(&format!("{{{{{}}}}}", key), value);
    }

    // Obtain the Transpose API key or return an error if not set
    let api_key = config.transpose_api_key().ok_or("Transpose API key not set")?;

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("X-API-KEY", api_key)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Transpose API request failed with status: {}", response.status()).into());
    }

    let result: Value = response.json().await?;
    
    if let Some(results) = result.get("results").and_then(|v| v.as_array()) {
        Ok(results.to_vec())
    } else {
        Err(format!("Unexpected API response: {:?}", result).into())
    }
}

pub async fn query_ethereum_account(config: &Config, address: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let sql_query = load_sql_query("ethereum_accounts.sql").await;
    query_transpose(config, &sql_query, &[("address", address)]).await
}

pub async fn query_ethereum_transactions(config: &Config, addresses: &[String]) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let sql_query = load_sql_query("ethereum_transactions.sql").await;
    let mut all_transactions = Vec::new();
    let mut last_request_time = Instant::now();

    for address in addresses {
        let mut offset = 0;
        let limit = 100;

        loop {
            // Ensure at least 1 second has passed since the last request
            let elapsed = last_request_time.elapsed();
            if elapsed < Duration::from_secs(1) {
                sleep(Duration::from_secs(1) - elapsed).await;
            }

            let limit_str = limit.to_string();
            let offset_str = offset.to_string();
            let params = vec![
                ("wallet_address", address.as_str()),
                ("limit", &limit_str),
                ("offset", &offset_str),
            ];

            let transactions = query_transpose(config, &sql_query, &params).await?;
            last_request_time = Instant::now();

            if transactions.is_empty() {
                break;
            }

            all_transactions.extend(transactions);
            offset += limit;

            // Check if we've reached the 1 MB response size limit (approximate)
            if all_transactions.len() * 1000 > 1_000_000 {
                println!("Warning: Reached approximate 1 MB response size limit. Some transactions may be missing.");
                break;
            }
        }
    }

    Ok(all_transactions)
}
