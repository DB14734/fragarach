/// Database schema initialization and management
/// 
/// # Tables
/// Creates the following tables:
/// - ethereum_accounts
/// - ethereum_transactions
/// - urlscan_domain_data
/// - urlscan_dom_snapshot
/// - urlscan_scan_data
/// 
/// # Schema Version
/// Current schema version: 1.0
use sqlx::{sqlite::SqlitePool, query};

pub async fn setup_database_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("Setting up ethereum_accounts table...");
    
    // Create ethereum_accounts table
    query(
        "CREATE TABLE IF NOT EXISTS ethereum_accounts (
            address TEXT PRIMARY KEY,
            created_timestamp TEXT,
            creator_address TEXT,
            last_active_timestamp TEXT,
            type TEXT
        )"
    ).execute(pool).await?;
    println!("ethereum_accounts table created successfully.");

    // Create ethereum_transactions table
    println!("Setting up ethereum_transactions table...");
    query(
        "CREATE TABLE IF NOT EXISTS ethereum_transactions (
            transaction_hash TEXT PRIMARY KEY,
            base_fee_per_gas NUMERIC,
            block_number INTEGER,
            contract_address TEXT,
            fees_burned NUMERIC,
            fees_rewarded NUMERIC,
            fees_saved NUMERIC,
            from_address TEXT,
            gas_limit NUMERIC,
            gas_price NUMERIC,
            gas_used NUMERIC,
            input TEXT,
            internal_failed_transaction_count INTEGER,
            internal_transaction_count INTEGER,
            log_count INTEGER,
            max_fee_per_gas NUMERIC,
            max_priority_fee_per_gas NUMERIC,
            nonce INTEGER,
            output TEXT,
            position INTEGER,
            timestamp TIMESTAMP,
            to_address TEXT,
            transaction_fee NUMERIC,
            type INTEGER,
            value NUMERIC
        )"
    ).execute(pool).await?;
    println!("ethereum_transactions table created successfully.");

    // Create URLScan tables
    println!("Setting up urlscan_domain_data table...");
    query(
        "CREATE TABLE IF NOT EXISTS urlscan_domain_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            domain TEXT,
            uuid TEXT UNIQUE,
            result_url TEXT,
            api_url TEXT,
            visibility TEXT,
            useragent TEXT,
            country TEXT,
            screenshot_path TEXT,
            asn TEXT,
            ip TEXT,
            title TEXT,
            verdict_score INTEGER,
            verdict_brands TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(pool).await?;

    println!("Setting up urlscan_dom_snapshot table...");
    query(
        "CREATE TABLE IF NOT EXISTS urlscan_dom_snapshot (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT UNIQUE,
            dom TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(pool).await?;

    println!("Setting up urlscan_scan_data table...");
    query(
        "CREATE TABLE IF NOT EXISTS urlscan_scan_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT UNIQUE,
            ip TEXT,
            data_links TEXT,
            page_asn TEXT,
            page_ip TEXT,
            page_country TEXT,
            page_title TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(pool).await?;

    Ok(())
}