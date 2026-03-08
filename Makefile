# Makefile - VantisOS
# Szybkie komendy dla deweloperów
# M - Mikro-Feedback & Makefile

.PHONY: help build test clean install docs setup dev

# Domyślny cel
.DEFAULT_GOAL := help

# Kolory terminala
GREEN  := \033[0;32m
YELLOW := \033[0;33m
RED    := \033[0;31m
BLUE   := \033[0;34m
WHITE  := \033[0;37m
NC     := \033[0m # No Color

# Pomoc
help: ## Show this help message
	@echo ''
	@echo '$(YELLOW)VantisOS - Quick Reference$(NC)'
	@echo ''
	@echo '$(GREEN)Quick Start:$(NC)'
	@echo '  make setup          - Initial setup'
	@echo '  make build          - Build the project'
	@echo '  make test           - Run tests'
	@echo '  make dev            - Start development'
	@echo ''
	@echo '$(GREEN)Development:$(NC)'
	@echo '  make clean          - Clean build artifacts'
	@echo '  make fmt            - Format code'
	@echo '  make lint           - Run linters'
	@echo '  make check          - Run all checks'
	@echo ''
	@echo '$(GREEN)Documentation:$(NC)'
	@echo '  make docs           - Build documentation'
	@echo '  make docs-serve     - Serve docs locally'
	@echo ''
	@echo '$(GREEN)Release:$(NC)'
	@echo '  make release        - Create release'
	@echo '  make changelog      - Generate changelog'
	@echo ''
	@echo '$(GREEN)Utilities:$(NC)'
	@echo '  make install        - Install dependencies'
	@echo '  make update         - Update dependencies'
	@echo '  make version        - Show version'
	@echo ''

# Instalacja i setup
setup: ## Initial setup
	@echo '$(BLUE)🚀 Setting up VantisOS...$(NC)'
	cargo install cargo-watch
	cargo install cargo-edit
	cargo install cargo-expand
	@echo '$(GREEN)✅ Setup complete!$(NC)'

install: ## Install dependencies
	@echo '$(BLUE)📦 Installing dependencies...$(NC)'
	cargo fetch
	@echo '$(GREEN)✅ Dependencies installed!$(NC)'

update: ## Update dependencies
	@echo '$(BLUE)🔄 Updating dependencies...$(NC)'
	cargo update
	@echo '$(GREEN)✅ Dependencies updated!$(NC)'

# Budowanie
build: ## Build the project
	@echo '$(BLUE)🔨 Building VantisOS...$(NC)'
	bash build_advanced_kernel.sh
	@echo '$(GREEN)✅ Build complete!$(NC)'

clean: ## Clean build artifacts
	@echo '$(BLUE)🧹 Cleaning...$(NC)'
	cargo clean
	rm -rf target/
	@echo '$(GREEN)✅ Clean complete!$(NC)'

# Testowanie
test: ## Run tests
	@echo '$(BLUE)🧪 Running tests...$(NC)'
	cargo test --workspace
	@echo '$(GREEN)✅ Tests passed!$(NC)'

check: ## Run all checks
	@echo '$(BLUE)✅ Running all checks...$(NC)'
	cargo check --workspace
	cargo clippy --workspace
	@echo '$(GREEN)✅ All checks passed!$(NC)'

# Formatowanie i linting
fmt: ## Format code
	@echo '$(BLUE)🎨 Formatting code...$(NC)'
	cargo fmt --all
	@echo '$(GREEN)✅ Code formatted!$(NC)'

lint: ## Run linters
	@echo '$(BLUE)🔍 Running linters...$(NC)'
	cargo clippy --workspace -- -D warnings
	@echo '$(GREEN)✅ Linting complete!$(NC)'

# Dokumentacja
docs: ## Build documentation
	@echo '$(BLUE)📚 Building documentation...$(NC)'
	cargo doc --workspace --no-deps --open
	@echo '$(GREEN)✅ Documentation built!$(NC)'

docs-serve: ## Serve docs locally
	@echo '$(BLUE)🌐 Serving documentation...$(NC)'
	cargo doc --workspace --no-deps --open
	@echo '$(GREEN)✅ Documentation served!$(NC)'

# Development
dev: ## Start development
	@echo '$(BLUE)🚀 Starting development...$(NC)'
	cargo watch -x 'run'
	@echo '$(GREEN)✅ Development started!$(NC)'

# Release
release: ## Create release
	@echo '$(BLUE)📦 Creating release...$(NC)'
	bash scripts/release_helper.sh
	@echo '$(GREEN)✅ Release created!$(NC)'

changelog: ## Generate changelog
	@echo '$(BLUE)📝 Generating changelog...$(NC)'
	bash scripts/changelog_generator.sh
	@echo '$(GREEN)✅ Changelog generated!$(NC)'

# Utilities
version: ## Show version
	@echo '$(BLUE)VantisOS Version:$(NC) v1.4.0 "AI Advanced Features"'
	@echo '$(BLUE)Release Date:$(NC) March 5, 2026'
	@echo '$(BLUE)Status:$(NC) ✅ Production Ready'

# Full check
all: fmt lint check test ## Run all checks and tests
	@echo '$(GREEN)✅ All checks and tests passed!$(NC)'