# R-Web API Server

A Rust web API server built with Actix-Web and SeaORM.

## Project Structure

```
src/
├── lib.rs              # Library exports
├── main.rs             # Backward compatibility notice
├── bin/
│   ├── server.rs       # Main web server binary
│   └── create_superuser.rs # Superuser creation utility
├── config/             # Configuration management
├── db/                 # Database initialization
├── handlers/           # Request handlers
├── routes/             # Route definitions
├── middleware/         # Custom middleware
└── state.rs            # Application state
```

## Running the Application

### Start the Web Server

```bash
# Development (default binary)
cargo run

# Or explicitly specify the server binary
cargo run --bin server

# Production build
cargo build --release
./target/release/server
```

### Create a Superuser

```bash
cargo run --bin create_superuser
```

### Available Endpoints

Once the server is running, you can access:

- **API Documentation**: http://localhost:8080/redoc
- **OpenAPI Spec**: http://localhost:8080/openapi.json
- **Scalar UI**: http://localhost:8080/scalar
- **Health Check**: http://localhost:8080/health

## Configuration

Configuration is managed through:
1. Default values in the code
2. Configuration files in `config/` directory
3. Environment variables

### Environment Variables

- `ENVIRONMENT` - Set to "development" or "production"
- `DB_HOST`, `DB_PORT`, `DB_NAME`, `DB_USER`, `DB_PASSWORD` - Database settings
- `APP_*` - Application-specific settings (use underscore for nested config)

### Configuration Files

- `config/development.toml` - Development environment settings
- `config/production.toml` - Production environment settings

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run --bin server

# Run database migrations
cargo run -p migration
```

## API Versioning

The API supports versioning through URL paths:
- `/api/v1/` - Version 1 endpoints
- `/api/v2/` - Version 2 endpoints (when available)

Version is configurable in the application settings.
