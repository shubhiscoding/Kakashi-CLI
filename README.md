# Kakashi

A smart environment manager for switching between different `.env` configurations.

## About

Kakashi is a lightweight CLI tool written in Rust that helps you manage multiple environment files (`.env.*`). Instead of manually editing `.env` files or juggling multiple configuration files, Kakashi lets you switch between pre-configured environments with a single command.

Perfect for managing different configurations for development, staging, and production environments, or for swapping between different sets of API keys and credentials.

## Installation

Clone the repository and install using Cargo:

```bash
cargo install --path .
```

This will compile and install the `kakashi` binary to your system.

## Usage

### Initialize

Set up Kakashi in your project:

```bash
kakashi init <path>
```

This creates a `.kakashi` directory to store configuration and metadata.

### List Environments

View all available environment files:

```bash
kakashi list
```

This displays all `.env.*` files in your current directory.

### Switch Environment

Switch to a specific environment:

```bash
kakashi switch <env>
```

This replaces your current `.env` file with the contents of `.env.<env>`. For example:

```bash
kakashi switch dev     # Loads .env.dev into .env
kakashi switch prod    # Loads .env.prod into .env
```

### Current Environment

Check which environment is currently active:

```bash
kakashi current
```

## Example Workflow

```bash
# Initialize Kakashi in your project
kakashi init .

# Create your environment files
echo "DB_HOST=localhost" > .env.dev
echo "API_KEY=dev_key_123" >> .env.dev

echo "DB_HOST=prod.example.com" > .env.prod
echo "API_KEY=prod_key_abc" >> .env.prod

# List available environments
kakashi list
# Output:
# .env.dev
# .env.prod

# Switch to development
kakashi switch dev

# Switch to production
kakashi switch prod

# Check current environment
kakashi current
```

## Project Structure

```
src/
├── main.rs           # CLI entry point and command routing
└── commands/
    ├── mod.rs        # Command module definitions
    ├── init.rs       # Initialize command
    ├── list.rs       # List environments command
    ├── switch.rs     # Switch environment command
    └── current.rs    # Show current environment command
```

## Dependencies

- `clap` - Command-line argument parser
