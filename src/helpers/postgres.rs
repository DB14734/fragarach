/// PostgreSQL database operations and schema management
/// 
/// # Schema
/// Implements tables for:
/// - Ethereum accounts
/// - Ethereum transactions
/// - URLScan domain data
/// - URLScan DOM snapshots
/// 
/// # Features
/// - Schema initialization
/// - Data persistence
/// - Upsert operations
use sqlx::postgres::PgPool;
use serde_json::Value;

pub async fn setup_postgres_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ethereum_accounts (
            address TEXT PRIMARY KEY,
            created_timestamp TEXT,
            creator_address TEXT,
            last_active_timestamp TEXT,
            type TEXT
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ethereum_transactions (
            transaction_hash TEXT PRIMARY KEY,
            base_fee_per_gas NUMERIC,
            block_number BIGINT,
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
            nonce BIGINT,
            output TEXT,
            position INTEGER,
            timestamp TIMESTAMP,
            to_address TEXT,
            transaction_fee NUMERIC,
            type INTEGER,
            value NUMERIC
        )"
    ).execute(pool).await?;

    // Add URLScan tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urlscan_domain_data (
            id SERIAL PRIMARY KEY,
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

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urlscan_dom_snapshot (
            id SERIAL PRIMARY KEY,
            uuid TEXT UNIQUE,
            dom TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urlscan_scan_data (
            id SERIAL PRIMARY KEY,
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

pub async fn save_to_postgres(pool: &PgPool, data: &[Value], table_name: &str) -> Result<(), sqlx::Error> {
    println!("Attempting to save {} records to PostgreSQL table: {}", data.len(), table_name);
    for (index, record) in data.iter().enumerate() {
        let columns = record.as_object().unwrap().keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        let placeholders = (1..=record.as_object().unwrap().len()).map(|i| format!("${}", i)).collect::<Vec<_>>().join(", ");
        
        let primary_key = if table_name == "ethereum_accounts" { "address" } else { "transaction_hash" };
        
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT ({}) DO UPDATE SET {}",
            table_name,
            columns,
            placeholders,
            primary_key,
            columns.split(", ")
                .enumerate()
                .map(|(i, col)| format!("{} = ${}", col, i + 1))
                .collect::<Vec<_>>()
                .join(", ")
        );
        
        println!("Executing SQL for record {}: {}", index, sql);
        
        let mut query = sqlx::query(&sql);
        for value in record.as_object().unwrap().values() {
            query = query.bind(value.as_str().unwrap_or(""));
        }
        
        match query.execute(pool).await {
            Ok(_) => println!("Successfully inserted/updated record {}", index),
            Err(e) => println!("Error inserting/updating record {}: {}", index, e),
        }
    }
    
    println!("Finished saving data to PostgreSQL");
    Ok(())
}