/// Main entry point for the Fragarach OSINT Framework
/// 
/// # Architecture
/// The application follows a modular architecture with the following components:
/// - API integrations (Transpose, URLScan)
/// - CLI interface
/// - Configuration management
/// - Database connections (DuckDB)
/// 
/// # Database Initialization
/// - Creates DuckDB database if it doesn't exist
/// 
/// # Error Handling
/// Implements comprehensive error handling for database connections and schema setup
mod api;
mod cli;
mod config;
mod helpers;

use config::Config;
use duckdb::Connection;
use std::fs;
use std::path::Path;

#[tokio::main]
/// Initializes the application, sets up database connections, and launches the CLI interface
///
/// # Errors
/// Returns an error if:
/// - Database directory creation fails
/// - Database connection fails
/// - Schema setup fails
/// - CLI execution fails
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    // Create the data directory if it doesn't exist
    fs::create_dir_all("data")?;
    
    // Initialize DuckDB connection
    let db_path = Path::new("data/fragarach.duckdb");
    let conn = Connection::open(db_path)?;

    // Initialize schema
    if let Err(e) = helpers::database_setup::setup_database_schema(&conn) {
        eprintln!("Error setting up database schema: {}", e);
    }

    cli::run_cli(&mut config, &conn).await?;

    Ok(())
}