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
use dialoguer::{theme::ColorfulTheme, Select, Input};
use console::Style;
use crate::config::Config;
use crate::api::transpose;
use crate::helpers::{database_setup, database_operations};
use duckdb::Connection;
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
                                          v0.1.1
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
    conn: &Connection,
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
                "⚙️  Setup",
                "🔍 Query Ethereum Account",
                "📊 Query Ethereum Transactions",
                "🌐 Scan Domain",
                "⚡ Settings",
                "🚪 Exit"
            ])
            .interact()?;

        match selection {
            0 => setup(config, conn).await?,
            1 => query_ethereum_account(config, conn).await?,
            2 => query_ethereum_transactions(config, conn).await?,
            3 => scan_domain(config, conn).await?,
            4 => settings_menu(config).await?,
            5 => {
                animate_text("SHUTTING DOWN FRAGARACH SYSTEMS...");
                thread::sleep(Duration::from_millis(500));
                println!("{}", "System offline! 👋".bright_magenta());
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

async fn setup(config: &mut Config, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    print_cyber_header("SYSTEM SETUP AND CONFIGURATION");

    print_cyber_step("01", "Configuring Database Schema");
    if let Err(e) = database_setup::setup_database_schema(conn) {
        println!("{} {}", "✘ Database schema setup failed:".bright_red(), e);
    } else {
        println!("{}", "✔ Database schema configured successfully.".bright_green());
    }

    print_cyber_step("02", "API Authentication Setup");
    if config.transpose_api_key().is_none() {
        set_transpose_api_key(config).await?;
    } else {
        println!("{}", "✔ Transpose API key already configured.".bright_green());
    }

    print_cyber_step("03", "URLScan Integration Setup");
    if config.urlscan_api_key().is_none() {
        set_urlscan_api_key(config).await?;
    } else {
        println!("{}", "✔ URLScan API key already configured.".bright_green());
    }

    println!("\n{}", CYBER_SEPARATOR.bright_blue());
    animate_text("SETUP SEQUENCE COMPLETE");
    Ok(())
}

async fn query_ethereum_account(config: &Config, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    if config.transpose_api_key().is_none() {
        println!("{}", "Transpose API key is not set. Please run 'setup' to set it.".red());
        return Ok(());
    }

    let address: String = Input::new()
        .with_prompt("Enter Ethereum address")
        .interact_text()?;

    println!("{}", "[Step 1] Querying Ethereum account details".yellow());
    let account_data = transpose::query_ethereum_account(config, &address).await?;

    println!("{}", "[Step 2] Saving data to database".yellow());
    if let Err(e) = database_operations::save_records(conn, &account_data, "ethereum_accounts") {
        println!("{} {}", "✘ Error saving data:".bright_red(), e);
    } else {
        println!("{}", "✔ Data saved successfully.".bright_green());
    }

    println!("{}", format!("\nRetrieved account data for address {}", address).green());
    Ok(())
}

async fn query_ethereum_transactions(config: &Config, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
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

    println!("{}", "[Step 2] Saving data to database".yellow());
    if let Err(e) = database_operations::save_records(conn, &transactions, "ethereum_transactions") {
        println!("{} {}", "✘ Error saving data:".bright_red(), e);
    } else {
        println!("{}", "✔ Data saved successfully.".bright_green());
    }

    println!("{}", format!("\nRetrieved and processed {} transactions for address {}", total_transactions, address).green());
    Ok(())
}

async fn scan_domain(config: &Config, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    if config.urlscan_api_key().is_none() {
        println!("{}", "URLScan API key is not set. Please run 'setup' to configure.".red());
        return Ok(());
    }

    let domain: String = Input::new()
        .with_prompt("Enter domain to scan")
        .interact_text()?;

    println!("{}", "[Step 1] Initiating domain scan".yellow());
    match crate::api::urlscan::scan_domain(config, &domain, conn).await {
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

    println!("\nDatabase: DuckDB");
    println!("└─ Location: data/fragarach.duckdb");

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Settings Menu")
        .default(0)
        .items(&[
            "🔌 Manage API Keys",
            "↩️  Back"
        ])
        .interact()?;

    match selection {
        0 => manage_integrations(config).await?,
        1 => return Ok(()),
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

async fn set_transpose_api_key(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = Input::new()
        .with_prompt("Enter your Transpose API key")
        .interact_text()?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(".env")?;

    writeln!(file, "TRANSPOSE_API_KEY={}", api_key)?;
    println!("{}", "Transpose API key saved successfully.".green());
    
    // Update the config with the new API key
    config.set_transpose_api_key(Some(api_key));

    Ok(())
}

async fn set_urlscan_api_key(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
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
    
    // Update the config with the new API key
    config.set_urlscan_api_key(Some(api_key));

    Ok(())
}
