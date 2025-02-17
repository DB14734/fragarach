/// DuckDB schema initialization and management
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
/// Current schema version: 1.1
use duckdb::{Connection, Result};

pub fn setup_database_schema(conn: &Connection) -> Result<()> {
    println!("Setting up ethereum_accounts table...");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS ethereum_accounts (
            address VARCHAR PRIMARY KEY,
            created_timestamp TIMESTAMP,
            creator_address VARCHAR,
            last_active_timestamp TIMESTAMP,
            type VARCHAR
        )"
    )?;
    println!("ethereum_accounts table created successfully.");

    println!("Setting up ethereum_transactions table...");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS ethereum_transactions (
            transaction_hash VARCHAR PRIMARY KEY,
            base_fee_per_gas DOUBLE,
            block_number BIGINT,
            contract_address VARCHAR,
            fees_burned DOUBLE,
            fees_rewarded DOUBLE,
            fees_saved DOUBLE,
            from_address VARCHAR,
            gas_limit DOUBLE,
            gas_price DOUBLE,
            gas_used DOUBLE,
            input TEXT,
            internal_failed_transaction_count INTEGER,
            internal_transaction_count INTEGER,
            log_count INTEGER,
            max_fee_per_gas DOUBLE,
            max_priority_fee_per_gas DOUBLE,
            nonce BIGINT,
            output TEXT,
            position INTEGER,
            timestamp TIMESTAMP,
            to_address VARCHAR,
            transaction_fee DOUBLE,
            type INTEGER,
            value DOUBLE
        )"
    )?;
    println!("ethereum_transactions table created successfully.");

    println!("Setting up urlscan tables...");
    conn.execute_batch(
        "CREATE SEQUENCE IF NOT EXISTS urlscan_domain_seq START 1;
         CREATE TABLE IF NOT EXISTS urlscan_domain_data (
            id BIGINT PRIMARY KEY DEFAULT nextval('urlscan_domain_seq'),
            domain VARCHAR,
            uuid VARCHAR UNIQUE,
            result_url VARCHAR,
            api_url VARCHAR,
            visibility VARCHAR,
            useragent VARCHAR,
            country VARCHAR,
            screenshot_path VARCHAR,
            asn VARCHAR,
            ip VARCHAR,
            title VARCHAR,
            verdict_score INTEGER,
            verdict_brands TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )?;
    println!("urlscan_domain_data table created successfully.");

    conn.execute_batch(
        "CREATE SEQUENCE IF NOT EXISTS urlscan_dom_seq START 1;
         CREATE TABLE IF NOT EXISTS urlscan_dom_snapshot (
            id BIGINT PRIMARY KEY DEFAULT nextval('urlscan_dom_seq'),
            uuid VARCHAR UNIQUE,
            dom TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )?;
    println!("urlscan_dom_snapshot table created successfully.");

    conn.execute_batch(
        "CREATE SEQUENCE IF NOT EXISTS urlscan_scan_seq START 1;
         CREATE TABLE IF NOT EXISTS urlscan_scan_data (
            id BIGINT PRIMARY KEY DEFAULT nextval('urlscan_scan_seq'),
            uuid VARCHAR UNIQUE,
            ip VARCHAR,
            data_links TEXT,
            page_asn VARCHAR,
            page_ip VARCHAR,
            page_country VARCHAR,
            page_title VARCHAR,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )?;
    println!("urlscan_scan_data table created successfully.");

    // Final confirmation
    println!("All URLScan tables created successfully.");

    Ok(())
} 