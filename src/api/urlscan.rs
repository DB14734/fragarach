/// URLScan API integration for domain scanning and analysis
/// 
/// # Features
/// - Domain scanning with private visibility
/// - Screenshot capture
/// - DOM snapshot storage
/// - Verdict analysis
/// 
/// # Database Integration
/// Supports both SQLite and PostgreSQL for storing:
/// - Scan results
/// - Domain data
/// - Screenshots
/// - DOM snapshots
use crate::config::Config;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;
use sqlx::{SqlitePool, PgPool};

#[derive(Debug, Serialize, Deserialize)]
struct ScanResponse {
    uuid: String,
    result: String,
    api: String,
    visibility: String,
    options: Option<ScanOptions>,
    country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScanOptions {
    useragent: Option<String>,
}

pub async fn scan_domain(
    config: &Config,
    domain: &str,
    sqlite_pool: Option<&SqlitePool>,
    pg_pool: Option<&PgPool>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Obtain the API key
    let api_key = config.urlscan_api_key().ok_or("URLScan API key not set")?;

    let client = Client::new();
    
    // Build headers for the request
    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
    headers.insert("API-Key", header::HeaderValue::from_str(&api_key)?);

    // Prepare request body: scan the domain with private visibility
    let body = serde_json::json!({
        "url": domain,
        "visibility": "private",
    });

    // Send initial scan request
    let initial_resp = client.post("https://urlscan.io/api/v1/scan/")
        .headers(headers.clone())
        .json(&body)
        .send()
        .await?;

    if !initial_resp.status().is_success() {
        return Err(format!("Initial URLScan request failed with status: {}", initial_resp.status()).into());
    }

    // Parse the initial response
    let initial_scan: ScanResponse = initial_resp.json().await?;
    let uuid = &initial_scan.uuid;
    println!("Scan initiated for domain {}. UUID: {}", domain, uuid);

    // Insert initial scan data to URLScan domain data table
    if let Some(pool) = sqlite_pool {
        sqlx::query(
            "INSERT OR REPLACE INTO urlscan_domain_data (
                domain, uuid, result_url, api_url, visibility, useragent, country
            ) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(domain)
        .bind(uuid)
        .bind(&initial_scan.result)
        .bind(&initial_scan.api)
        .bind(&initial_scan.visibility)
        .bind(initial_scan.options.as_ref()
            .and_then(|opt| opt.useragent.as_deref())
            .unwrap_or("N/A"))
        .bind(initial_scan.country.as_deref().unwrap_or("N/A"))
        .execute(pool)
        .await?;
    }
    if let Some(pool) = pg_pool {
        sqlx::query(
            "INSERT INTO urlscan_domain_data (
                domain, uuid, result_url, api_url, visibility, useragent, country
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (uuid) DO UPDATE SET
                result_url = EXCLUDED.result_url,
                api_url = EXCLUDED.api_url,
                visibility = EXCLUDED.visibility,
                useragent = EXCLUDED.useragent,
                country = EXCLUDED.country"
        )
        .bind(domain)
        .bind(uuid)
        .bind(&initial_scan.result)
        .bind(&initial_scan.api)
        .bind(&initial_scan.visibility)
        .bind(initial_scan.options.as_ref()
            .and_then(|opt| opt.useragent.as_deref())
            .unwrap_or("N/A"))
        .bind(initial_scan.country.as_deref().unwrap_or("N/A"))
        .execute(pool)
        .await?;
    }

    // Poll until the full scan result is available (timeout after 120 secs)
    let full_scan: Value = {
        let mut elapsed = Duration::from_secs(0);
        let timeout = Duration::from_secs(120);
        let mut result_opt = None;
        while elapsed < timeout {
            let result_url = format!("https://urlscan.io/api/v1/result/{}/", uuid);
            let res = client.get(&result_url).send().await?;
            if res.status() == reqwest::StatusCode::OK {
                result_opt = Some(res.json::<Value>().await?);
                break;
            } else if res.status() == reqwest::StatusCode::NOT_FOUND {
                println!("Scan not finished yet, retrying in 5 seconds...");
                sleep(Duration::from_secs(5)).await;
                elapsed += Duration::from_secs(5);
            } else {
                return Err(format!("Failed to retrieve scan result. Status: {}", res.status()).into());
            }
        }
        result_opt.ok_or("Timeout waiting for scan to complete.")?
    };

    // Extract fields from full scan result
    let default_page = serde_json::Map::new();
    let page = full_scan.get("page")
        .and_then(|p| p.as_object())
        .unwrap_or(&default_page);

    let default_verdicts = serde_json::Map::new();
    let verdicts = full_scan.get("verdicts")
        .and_then(|v| v.get("urlscan"))
        .and_then(|v| v.as_object())
        .unwrap_or(&default_verdicts);

    let asn = page.get("asn").and_then(|v| v.as_str()).unwrap_or("N/A");
    let ip = page.get("ip").and_then(|v| v.as_str()).unwrap_or("N/A");
    let title = page.get("title").and_then(|v| v.as_str()).unwrap_or("N/A");
    let verdict_score = verdicts.get("score").map(|v| v.to_string()).unwrap_or("N/A".to_string());
    let verdict_brands = verdicts.get("brands").map(|v| v.to_string()).unwrap_or("[]".to_string());

    // Update the domain data record with full scan details
    if let Some(pool) = sqlite_pool {
        sqlx::query(
            "UPDATE urlscan_domain_data
             SET asn = ?, ip = ?, title = ?, verdict_score = ?, verdict_brands = ?
             WHERE uuid = ?"
        )
        .bind(asn)
        .bind(ip)
        .bind(title)
        .bind(&verdict_score)
        .bind(&verdict_brands)
        .bind(uuid)
        .execute(pool)
        .await?;
    }
    if let Some(pool) = pg_pool {
        sqlx::query(
            "UPDATE urlscan_domain_data
             SET asn = $1, ip = $2, title = $3, verdict_score = $4, verdict_brands = $5
             WHERE uuid = $6"
        )
        .bind(asn)
        .bind(ip)
        .bind(title)
        .bind(&verdict_score)
        .bind(&verdict_brands)
        .bind(uuid)
        .execute(pool)
        .await?;
    }

    // Download the screenshot from URLScan
    let screenshot_url = format!("https://urlscan.io/screenshots/{}.png", uuid);
    let screenshot_resp = client.get(&screenshot_url).send().await?;
    if !screenshot_resp.status().is_success() {
        println!("Failed to download screenshot for UUID: {}", uuid);
    }
    let screenshot_bytes = screenshot_resp.bytes().await?;
    let screenshots_dir = "screenshots";
    tokio::fs::create_dir_all(screenshots_dir).await?;
    let screenshot_path = format!("{}/{}.png", screenshots_dir, uuid);
    tokio::fs::write(&screenshot_path, &screenshot_bytes).await?;

    // Update record with screenshot path
    if let Some(pool) = sqlite_pool {
        sqlx::query("UPDATE urlscan_domain_data SET screenshot_path = ? WHERE uuid = ?")
            .bind(&screenshot_path)
            .bind(uuid)
            .execute(pool)
            .await?;
    }
    if let Some(pool) = pg_pool {
        sqlx::query("UPDATE urlscan_domain_data SET screenshot_path = $1 WHERE uuid = $2")
            .bind(&screenshot_path)
            .bind(uuid)
            .execute(pool)
            .await?;
    }

    // Retrieve the DOM snapshot and store it
    let dom_url = format!("https://urlscan.io/dom/{}/", uuid);
    let dom_resp = client.get(&dom_url).send().await?;
    let dom_data = dom_resp.text().await?;
    if let Some(pool) = sqlite_pool {
        sqlx::query("INSERT OR REPLACE INTO urlscan_dom_snapshot (uuid, dom) VALUES (?, ?)")
            .bind(uuid)
            .bind(&dom_data)
            .execute(pool)
            .await?;
    }
    if let Some(pool) = pg_pool {
        sqlx::query(
            "INSERT INTO urlscan_dom_snapshot (uuid, dom) VALUES ($1, $2)
             ON CONFLICT (uuid) DO UPDATE SET dom = EXCLUDED.dom"
        )
        .bind(uuid)
        .bind(&dom_data)
        .execute(pool)
        .await?;
    }

    println!("Domain {} scanned successfully.", domain);
    Ok(())
} 