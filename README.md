# UTOTool Rust Bot

A modular Telegram bot built with Rust, featuring a pig farming game and extensible module system.

## Features

- 🐷 **Pig Game Module** - Create, feed, and manage virtual pigs
- 🗄️ **PostgreSQL Database** - Persistent data storage with migrations
- ⚙️ **YAML Configuration** - Easy configuration management
- 🔧 **Modular Architecture** - Add new features as separate modules
- 🌍 **Unicode Support** - Full support for non-Latin characters with case-insensitive search

## Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL
- Telegram Bot Token

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd utotool-rust
   ```

2. **Set up PostgreSQL**
  # Start PostgreSQL service
   ```bash
     brew services start postgresql  # macOS
   ```
   ```bash
     sudo systemctl start postgresql  # Linux
   ```
     # Create database
  ```bash
     psql postgres
     CREATE DATABASE botdb;
     \q
  ```
3. **Configure environment variables**
   ```bash
      export TELEGRAM_TOKEN="your_bot_token_here"
   ```
4. **Update configuration**
```yaml
  # config.yaml
  game:
    FEED_DELAY: 4
    SALO_DELAY: 8
    MAX_ITEMS: 15
    BASE_PILLS_CHANCE: 0.33
    BASE_PILLS_CHANCE_GROW: 0.75

  database_url: ${database_url}
```
5. **Start the bot**
```bash
   cargo run
```
