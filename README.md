<h1 align="center">Fragarach</h1>

<p align="center">
  <b>Modular OSINT framework for blockchain forensics and investigations</b>
</p>

## Overview

Fragarach is a modular OSINT framework designed for blockchain investigations and forensics, implemented in Rust. The framework provides a comprehensive suite of tools for building and analyzing blockchain intelligence data lakes.

## Features & Integrations

### Current Integrations
- **Transpose API**
  - Ethereum blockchain data retrieval

- **URLScan API**
  - Domain scanning with private visibility
  - Screenshot capture
  - DOM snapshot storage

### Supported Networks
- **Ethereum**
  - Account analysis
- **Bitcoin** (Under Development)
- **Solana** (Planned)

## Installation & Setup

### Prerequisites

1. **Rust Installation**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Database Requirements**
   - SQLite (default)
   - PostgreSQL (optional)

### Installation Methods

1. **Via Cargo**
   ```bash
   cargo install fragarach
   ```

2. **From Source**
   ```bash
   git clone https://github.com/BD14734/fragarach.git
   cd fragarach
   cargo build --release
   ```

## Usage

### Configuration
1. **Environment Setup**
   ```bash
   fragarach setup
   ```
   This will create a `config.json` and `.env` file with:
   - `SAVE_AS_SQLITE`: SQLite storage toggle (default: true)
   - `SAVE_AS_POSTGRES`: PostgreSQL storage toggle (default: false)
   - `TRANSPOSE_API_KEY`: Transpose API authentication
   - `URLSCAN_API_KEY`: URLScan API authentication
   - `POSTGRES_URL`: PostgreSQL connection string (if enabled)

2. **Database Configuration**
   - SQLite: Automatically creates `data/sqlite/fragarach.db`
   - PostgreSQL: Requires valid connection string in `.env`

### Dependencies

Core dependencies:
- `tokio`: Async runtime and utilities
- `sqlx`: Database operations
- `reqwest`: HTTP client
- `serde`: Serialization/deserialization
- `clap`: CLI argument parsing
- `dotenv`: Environment variable management

UI dependencies:
- `colored`: Terminal coloring
- `dialoguer`: Interactive prompts
- `console`: Terminal utilities

### Core Components

#### Storage Layer
Supports dual database architecture:
- **SQLite**: Local storage implementation with:
  - Data persistence
  - Record updates
  - Batch operations

- **PostgreSQL**: Xata Cloud storage implementation featuring:
  - Schema initialization
  - Data persistence
  - Upsert operations

#### Schema Design

Common tables across both database systems:
1. `ethereum_accounts`
   - Primary account information
   - Creation timestamps
   - Activity tracking
   - Account typing

2. `ethereum_transactions`
   - Transaction details
   - Gas metrics
   - Fee calculations
   - Internal transaction tracking

3. `urlscan_domain_data`
   - Domain scan results
   - Verdict analysis
   - Screenshot references
   - Geographical data

4. `urlscan_dom_snapshot`
   - DOM state storage
   - Temporal tracking
   - UUID referencing

## Contributing

### Development Setup
1. **Fork & Clone**
   ```bash
   git clone https://github.com/your-username/fragarach.git
   cd fragarach
   ```

2. **Build**
   ```bash
   cargo build
   ```

3. **Environment Configuration**
   ```bash
   cp .env.example .env
   # Edit .env with your API keys
   ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
