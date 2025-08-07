# Load environment variables
include .env
export

# Construct DATABASE_URL from individual parameters
export DATABASE_URL = postgres://$(DB_USER):$(DB_PASSWORD)@$(DB_HOST):$(DB_PORT)/$(DB_NAME)

# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*##"; printf "\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: dev
dev: ## Run development server with auto-reload
	cargo watch -x run

.PHONY: run
run: ## Run the application
	cargo run

.PHONY: build
build: ## Build the application
	cargo build

.PHONY: build-release
build-release: ## Build release version
	cargo build --release

.PHONY: test
test: ## Run tests
	cargo test

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

.PHONY: check
check: ## Check code for errors without building
	cargo check

.PHONY: fmt
fmt: ## Format code
	cargo fmt

.PHONY: lint
lint: ## Run clippy linter
	cargo clippy -- -D warnings

.PHONY: install-tools
install-tools: ## Install development tools
	cargo install cargo-watch
	cargo install sea-orm-cli

# Database related commands
.PHONY: db-create
db-create: ## Create database
	@echo "Creating database $(DB_NAME)..."
	@PGPASSWORD=$(DB_PASSWORD) psql -h $(DB_HOST) -p $(DB_PORT) -U $(DB_USER) -d postgres -c "CREATE DATABASE $(DB_NAME);" || true

.PHONY: db-drop
db-drop: ## Drop database
	@echo "Dropping database $(DB_NAME)..."
	@PGPASSWORD=$(DB_PASSWORD) psql -h $(DB_HOST) -p $(DB_PORT) -U $(DB_USER) -d postgres -c "DROP DATABASE IF EXISTS $(DB_NAME);"

.PHONY: db-reset
db-reset: db-drop db-create ## Reset database (drop and create)
	@echo "Database reset complete"

# SeaORM related commands
.PHONY: entity-generate
entity-generate: ## Generate entities from database
	sea-orm-cli generate entity -o entity/src

.PHONY: migrate-init
migrate-init: ## Initialize migrations
	sea-orm-cli migrate init

.PHONY: migrate-generate
migrate-generate: ## Generate a new migration (usage: make migrate-generate name=create_users_table)
	sea-orm-cli migrate generate $(name)

.PHONY: migrate-up
migrate-up: ## Run pending migrations
	sea-orm-cli migrate up

.PHONY: migrate-down
migrate-down: ## Rollback last migration
	sea-orm-cli migrate down

.PHONY: migrate-fresh
migrate-fresh: ## Drop all tables and re-run migrations
	sea-orm-cli migrate fresh

.PHONY: migrate-status
migrate-status: ## Show migration status
	sea-orm-cli migrate status

# Combined commands
.PHONY: setup
setup: install-tools db-create ## Initial project setup

.PHONY: fresh
fresh: clean db-reset build ## Clean everything and start fresh
	@echo "Fresh start complete!"