# Create Superuser Binary

This binary creates a superuser account for the service application.

## Usage

```bash
# Build the binary
cargo build --bin create_superuser

# Run the binary
cargo run --bin create_superuser

# Or run the built binary directly
./target/debug/create_superuser
```

## Environment Variables

The binary uses the same configuration as the main application. You can set these environment variables:

- `ENVIRONMENT` - Set to "development" or "production" (default: "development")
- `DB_HOST` - Database host (default: "localhost")
- `DB_PORT` - Database port (default: 5432)
- `DB_NAME` - Database name (default: "rust_api_db")
- `DB_USER` - Database username (default: "postgres")
- `DB_PASSWORD` - Database password (default: "password")

Or create configuration files in the `config/` directory:
- `config/development.toml`
- `config/production.toml`

## Example

```bash
# Set environment variables and run
DB_HOST=localhost DB_NAME=mydb cargo run --bin create_superuser
```
