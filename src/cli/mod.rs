/// CLI interface implementation for the Fragarach framework
///
/// # Features
/// - Interactive menu system
/// - Colored output
/// - Progress animations
/// - Configuration management
/// - Database operations
///
/// # Menu Options
/// - System Setup
/// - Ethereum Account Query
/// - Ethereum Transaction Query
/// - Domain Scanning
/// - Settings Management
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select, Input, MultiSelect};
use console::Style;
use crate::config::Config;
use crate::api::transpose;
use crate::helpers::storage;
use crate::helpers::setup_schema;
use crate::helpers::postgres;
use sqlx::SqlitePool;
use sqlx::postgres::PgPool;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

const FRAGARACH_LOGO: &str = r#"
    ___                                    _
    | __>_ _  ___  ___  ___  _ _  ___  ___ | |_
    | _>| '_><_> |/ . |<_> || '_><_> |/ | '| . |
    |_| |_|  <___|\_. |<___||_|  <___|\_|_.|_|_|
                  <___'
                                          v0.1.0
"#;

const CYBER_BORDER: &str = "═══════════════════════════════════════════════════════════════════════════════";
const CYBER_SEPARATOR: &str = "───────────────────────────────────────────────────────────────────────────────";

fn print_cyber_header(text: &str) {
    println!("\n{}", CYBER_BORDER.bright_blue());
    println!("  {}", text.bright_cyan());
    println!("{}\n", CYBER_BORDER.bright_blue());
}

fn print_cyber_step(step: &str, text: &str) {
    println!("\n>> {} {}", format!("[{}]", step).bright_yellow(), text.bright_green());
}

fn animate_text(text: &str) {
    print!("\r");
    for (i, c) in text.chars().enumerate() {
        print!("{}", c.to_string().bright_cyan());
        if i % 2 == 0 {
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(25));
        }
    }
    println!();
}

pub async fn run_cli(
    config: &mut Config,
    sqlite_pool: Option<&SqlitePool>,
    pg_pool: Option<&PgPool>
) -> Result<(), Box<dyn std::error::Error>> {
    // Animated startup sequence
    println!("{}", CYBER_BORDER.bright_blue());
    animate_text("INITIALIZING FRAGARACH SYSTEMS...");
    thread::sleep(Duration::from_millis(500));
    println!("{}", FRAGARACH_LOGO.bright_magenta());
    animate_text("BLOCKCHAIN INVESTIGATION TOOLKIT ACTIVE");
    println!("{}", CYBER_BORDER.bright_blue());

    if config.transpose_api_key().is_none() {
        println!("\n{}", "[!] WARNING: Transpose API key not detected. Run 'setup' to configure.".bright_red());
    }

    if config.urlscan_api_key().is_none() {
        println!("{}", "[!] WARNING: URLScan API key not detected. Run 'setup' to configure.".bright_red());
    }

    let custom_theme = ColorfulTheme {
        defaults_style: Style::new().cyan(),
        prompt_style: Style::new().yellow(),
        prompt_prefix: Style::new().yellow().apply_to(">>".to_string()),
        prompt_suffix: Style::new().yellow().apply_to("::".to_string()),
        success_prefix: Style::new().green().apply_to("✔".to_string()),
        success_suffix: Style::new().green().apply_to("".to_string()),
        error_prefix: Style::new().red().apply_to("✘".to_string()),
        error_style: Style::new().red(),
        hint_style: Style::new().black().bright(),
        values_style: Style::new().blue(),
        active_item_style: Style::new().cyan(),
        inactive_item_style: Style::new().black().bright(),
        active_item_prefix: Style::new().cyan().apply_to("❯".to_string()),
        inactive_item_prefix: Style::new().black().bright().apply_to(" ".to_string()),
        checked_item_prefix: Style::new().green().apply_to("✔".to_string()),
        unchecked_item_prefix: Style::new().black().bright().apply_to("✘".to_string()),
        picked_item_prefix: Style::new().yellow().apply_to("❯".to_string()),
        unpicked_item_prefix: Style::new().black().bright().apply_to(" ".to_string()),
    };

    loop {
        println!("\n{}", CYBER_SEPARATOR.bright_blue());
        let selection = Select::with_theme(&custom_theme)
            .with_prompt("SELECT OPERATION MODE")
            .default(0)
            .items(&[
                "⚙️  System Setup",
                "🔍 Query Ethereum Account",
                "📊 Query Ethereum Transactions",
                "🌐 Scan Domain",
                "⚡ Settings",
                "✖️  Exit System"
            ])
            .interact()?;

        match selection {
            0 => setup(config, sqlite_pool, pg_pool).await?,
            1 => query_ethereum_account(config, sqlite_pool, pg_pool).await?,
            2 => query_ethereum_transactions(config, sqlite_pool, pg_pool).await?,
            3 => scan_domain_menu(config, sqlite_pool, pg_pool).await?,
            4 => settings_menu(config).await?,
            5 => {
                animate_text("SHUTTING DOWN FRAGARACH SYSTEMS...");
                thread::sleep(Duration::from_millis(500));
                println!("{}", "System offline! 👋".bright_magenta());
                break;
            }
            _ => unreachable!(),
        }

        config.save()?;
    }

    Ok(())
}

async fn setup(config: &mut Config, sqlite_pool: Option<&SqlitePool>, pg_pool: Option<&PgPool>) -> Result<(), Box<dyn std::error::Error>> {
    print_cyber_header("SYSTEM SETUP SEQUENCE INITIATED");

    print_cyber_step("01", "Configuring Database Schema");
    if let Some(pool) = sqlite_pool {
        setup_schema::setup_database_schema(pool).await?;
        println!("{}", "✔ SQLite database schema configured successfully.".bright_green());
    }

    if let Some(pool) = pg_pool {
        match postgres::setup_postgres_schema(pool).await {
            Ok(_) => println!("{}", "✔ PostgreSQL database schema configured successfully.".bright_green()),
            Err(e) => println!("{} {}", "✘ PostgreSQL schema setup failed:".bright_red(), e),
        }
    }

    print_cyber_step("02", "API Authentication Setup");
    if config.transpose_api_key().is_none() {
        set_transpose_api_key(config).await?;
    } else {
        println!("{}", "✔ Transpose API key already configured.".bright_green());
    }

    print_cyber_step("03", "URLScan Integration Setup");
    if config.urlscan_api_key().is_none() {
        let api_key: String = Input::new()
            .with_prompt("Enter your URLScan API key")
            .interact_text()?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(".env")?;

        writeln!(file, "URLSCAN_API_KEY={}", api_key)?;
        println!("{}", "✔ URLScan API key configured successfully.".bright_green());
    } else {
        println!("{}", "✔ URLScan API key already configured.".bright_green());
    }

    print_cyber_step("04", "Storage Configuration");
    let storage_options = vec!["🗄️ SQLite Database", "🌐 PostgreSQL Cloud"];
    let storage_selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select data storage methods")
        .items(&storage_options)
        .interact()?;

    config.save_as_sqlite = storage_selections.contains(&0);
    config.save_as_postgres = storage_selections.contains(&1);

    if config.save_as_postgres {
        print_cyber_step("05", "Testing PostgreSQL Connection");
        match set_postgres_credentials(config).await {
            Ok(_) => println!("{}", "✔ PostgreSQL connection test successful.".bright_green()),
            Err(e) => println!("{} {}", "✘ PostgreSQL connection test failed:".bright_red(), e),
        }
    }

    println!("\n{}", CYBER_SEPARATOR.bright_blue());
    animate_text("SETUP SEQUENCE COMPLETE");
    Ok(())
}

async fn set_postgres_credentials(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_id: String = Input::new().with_prompt("Enter your PostgreSQL workspace ID").interact_text()?;
    let api_key: String = Input::new().with_prompt("Enter your PostgreSQL API key").interact_text()?;
    let region: String = Input::new().with_prompt("Enter your PostgreSQL region").interact_text()?;
    let database_name: String = Input::new().with_prompt("Enter your PostgreSQL database name").interact_text()?;
    let branch_name: String = Input::new().with_prompt("Enter your PostgreSQL branch name").interact_text()?;

    let postgres_url = format!(
        "postgresql://{}:{}@{}.sql.xata.sh:5432/{}:{}?sslmode=require",
        workspace_id, api_key, region, database_name, branch_name
    );

    // Save the PostgreSQL URL to the .env file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(".env")?;

    writeln!(file, "POSTGRES_URL={}", postgres_url)?;

    env::set_var("POSTGRES_URL", &postgres_url);
    config.save_as_postgres = true;
    println!("{}", "PostgreSQL credentials saved successfully.".green());

    Ok(())
}

async fn query_ethereum_account(config: &Config, sqlite_pool: Option<&SqlitePool>, pg_pool: Option<&PgPool>) -> Result<(), Box<dyn std::error::Error>> {
    if config.transpose_api_key().is_none() {
        println!("{}", "Transpose API key is not set. Please run 'setup' to set it.".red());
        return Ok(());
    }

    let address: String = Input::new()
        .with_prompt("Enter Ethereum address")
        .interact_text()?;

    println!("{}", "[Step 1] Querying Ethereum account details".yellow());
    let account_data = transpose::query_ethereum_account(config, &address).await?;

    if config.save_as_sqlite {
        if let Some(pool) = sqlite_pool {
            println!("{}", "[Step 2] Saving data to SQLite".yellow());
            storage::save_to_sqlite(pool, &account_data, "ethereum_accounts").await?;
        } else {
            println!("SQLite pool is not available. Skipping SQLite save.");
        }
    }

    if config.save_as_postgres {
        if let Some(pool) = pg_pool {
            println!("{}", "[Step 3] Saving data to PostgreSQL".yellow());
            match postgres::save_to_postgres(pool, &account_data, "ethereum_accounts").await {
                Ok(_) => println!("Data saved to PostgreSQL successfully."),
                Err(e) => eprintln!("Error saving data to PostgreSQL: {}", e),
            }
        } else {
            println!("PostgreSQL pool is not available. Skipping PostgreSQL save.");
        }
    }

    println!("{}", format!("\nRetrieved account data for address {}", address).green());
    Ok(())
}

async fn query_ethereum_transactions(config: &Config, sqlite_pool: Option<&SqlitePool>, pg_pool: Option<&PgPool>) -> Result<(), Box<dyn std::error::Error>> {
    if config.transpose_api_key().is_none() {
        println!("{}", "Transpose API key is not set. Please run 'setup' to set it.".red());
        return Ok(());
    }

    let address: String = Input::new()
        .with_prompt("Enter Ethereum address")
        .interact_text()?;

    println!("{}", "[Step 1] Querying Ethereum transactions".yellow());
    let transactions = transpose::query_ethereum_transactions(config, &[address.clone()]).await?;

    if transactions.is_empty() {
        println!("{}", "No transactions found for the provided address".yellow());
        return Ok(());
    }

    let total_transactions = transactions.len();

    if config.save_as_sqlite {
        if let Some(pool) = sqlite_pool {
            println!("{}", "[Step 2] Saving data to SQLite".yellow());
            storage::save_to_sqlite(pool, &transactions, "ethereum_transactions").await?;
        } else {
            println!("SQLite pool is not available. Skipping SQLite save.");
        }
    }

    if config.save_as_postgres {
        if let Some(pool) = pg_pool {
            println!("{}", "[Step 3] Saving data to PostgreSQL".yellow());
            postgres::save_to_postgres(pool, &transactions, "ethereum_transactions").await?;
        } else {
            println!("PostgreSQL pool is not available. Skipping PostgreSQL save.");
        }
    }

    println!("{}", format!("\nRetrieved and processed {} transactions for address {}", total_transactions, address).green());
    Ok(())
}

async fn scan_domain_menu(
    config: &Config,
    sqlite_pool: Option<&SqlitePool>,
    pg_pool: Option<&PgPool>
) -> Result<(), Box<dyn std::error::Error>> {
    if config.urlscan_api_key().is_none() {
        println!("{}", "URLScan API key is not set. Please run 'setup' to configure.".red());
        return Ok(());
    }

    let domain: String = Input::new()
        .with_prompt("Enter domain to scan")
        .interact_text()?;

    println!("{}", "[Step 1] Initiating domain scan".yellow());
    match crate::api::urlscan::scan_domain(config, &domain, sqlite_pool, pg_pool).await {
        Ok(_) => println!("{}", format!("\nDomain scan completed for {}", domain).green()),
        Err(e) => println!("{}", format!("Error scanning domain: {}", e).red()),
    }

    Ok(())
}

async fn settings_menu(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nCurrent Settings:");
    println!("\nAPI Integrations:");
    println!("├─ Transpose API: {}", if config.transpose_api_key().is_some() {
        "✅ Active".green()
    } else {
        "❌ API key not detected".red()
    });
    println!("└─ URLScan API: {}", if config.urlscan_api_key().is_some() {
        "✅ Active".green()
    } else {
        "❌ API key not detected".red()
    });

    println!("\nStorage Configuration:");
    println!("├─ SQLite: {}", if config.save_as_sqlite { "Enabled".green() } else { "Disabled".red() });
    println!("└─ PostgreSQL: {}", if config.save_as_postgres { "Enabled".green() } else { "Disabled".red() });

    if config.save_as_postgres {
        println!("\nPostgreSQL Connection: {}", if config.postgres_url().is_some() {
            "✅ Configured".green()
        } else {
            "❌ Not Configured".red()
        });
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Settings")
        .default(0)
        .items(&[
            "🔌 Manage Integrations",
            "💾 Configure Storage Options",
            "↩️  Back"
        ])
        .interact()?;

    match selection {
        0 => manage_integrations(config).await?,
        1 => configure_storage_options(config).await?,
        2 => return Ok(()),
        _ => unreachable!(),
    }

    Ok(())
}

async fn manage_integrations(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nCurrent Integration Status:");
    println!("Transpose API: {}", if config.transpose_api_key().is_some() {
        "✅ Active".green()
    } else {
        "❌ API key not detected".red()
    });
    println!("URLScan API: {}", if config.urlscan_api_key().is_some() {
        "✅ Active".green()
    } else {
        "❌ API key not detected".red()
    });

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Integration to Configure")
        .default(0)
        .items(&[
            "🔑 Configure Transpose API",
            "🔑 Configure URLScan API",
            "↩️  Back"
        ])
        .interact()?;

    match selection {
        0 => set_transpose_api_key(config).await?,
        1 => set_urlscan_api_key(config).await?,
        2 => return Ok(()),
        _ => unreachable!(),
    }

    Ok(())
}

async fn set_transpose_api_key(_config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = Input::new().with_prompt("Enter your Transpose API key").interact_text()?;

    // Save the API key to the .env file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(".env")?;

    writeln!(file, "TRANSPOSE_API_KEY={}", api_key)?;

    println!("{}", "Transpose API key saved successfully.".green());

    Ok(())
}

async fn set_urlscan_api_key(_config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = Input::new()
        .with_prompt("Enter your URLScan API key")
        .interact_text()?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(".env")?;

    writeln!(file, "URLSCAN_API_KEY={}", api_key)?;
    println!("{}", "✅ URLScan API key saved successfully.".green());

    Ok(())
}

async fn configure_storage_options(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let storage_options = vec!["SQLite", "PostgreSQL"];
    let mut initial_selection = vec![false, false];
    if config.save_as_sqlite { initial_selection[0] = true; }
    if config.save_as_postgres { initial_selection[1] = true; }

    let storage_selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select how you would like to store the data (use space to select multiple)")
        .items(&storage_options)
        .defaults(&initial_selection)
        .interact()?;

    config.save_as_sqlite = storage_selections.contains(&0);
    config.save_as_postgres = storage_selections.contains(&1);

    if config.save_as_postgres && config.postgres_url().is_none() {
        set_postgres_credentials(config).await?;
    }

    config.save()?;
    Ok(())
}
