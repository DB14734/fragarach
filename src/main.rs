/// Main entry point for the Fragarach OSINT Framework
/// 
/// # Architecture
/// The application follows a modular architecture with the following components:
/// - API integrations (Transpose, URLScan)
/// - CLI interface
/// - Configuration management
/// - Database connections (SQLite, PostgreSQL)
/// 
/// # Database Initialization
/// - Creates SQLite database if enabled
/// - Establishes PostgreSQL connection if configured
/// 
/// # Error Handling
/// Implements comprehensive error handling for database connections and schema setup
mod api;
mod cli;
mod config;
mod helpers;

use config::Config;
use sqlx::sqlite::SqlitePool;
use sqlx::postgres::PgPool;
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

    let sqlite_pool = if config.save_as_sqlite {
        // Create the data/sqlite directory if it doesn't exist
        fs::create_dir_all("data/sqlite")?;
        
        // Check if database file exists, if not, create it
        let db_path = Path::new("data/sqlite/fragarach.db");
        if !db_path.exists() {
            // Touch the file to create it
            fs::File::create(db_path)?;
        }

        match SqlitePool::connect("sqlite:data/sqlite/fragarach.db").await {
            Ok(pool) => {
                // Initialize schema immediately after connecting
                if let Err(e) = crate::helpers::setup_schema::setup_database_schema(&pool).await {
                    eprintln!("Error setting up database schema: {}", e);
                }
                Some(pool)
            },
            Err(e) => {
                eprintln!("Error connecting to SQLite: {}", e);
                None
            }
        }
    } else {
        None
    };

    let pg_pool = if config.save_as_postgres {
        if let Some(postgres_url) = config.postgres_url() {
            match PgPool::connect(&postgres_url).await {
                Ok(pool) => Some(pool),
                Err(e) => {
                    eprintln!("Error connecting to PostgreSQL: {}", e);
                    None
                }
            }
        } else {
            eprintln!("PostgreSQL URL not set");
            None
        }
    } else {
        None
    };

    cli::run_cli(&mut config, sqlite_pool.as_ref(), pg_pool.as_ref()).await?;

    Ok(())
}