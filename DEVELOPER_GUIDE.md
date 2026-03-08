# VantisOS Developer Guide

## Development Environment Setup

### Prerequisites

- Rust 1.75 or later
- Cargo
- Git
- Build essentials

### Clone and Build

```bash
# Clone repository
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS

# Build
cargo build

# Run tests
cargo test

# Run
cargo run
```

### Development Container

Use VS Code Dev Containers for a complete environment:

1. Install Docker
2. Install VS Code
3. Open project in container

## Project Structure

```
VantisOS/
├── src/           # Source code
│   ├── verified/  # Formally verified code
│   └── ...
├── tests/         # Test suites
├── docs/          # Documentation
├── docker/        # Docker configurations
├── scripts/       # Utility scripts
└── iso_build/     # ISO build tools
```

## Coding Standards

### Rust Style

```bash
# Format code
cargo fmt

# Check with clippy
cargo clippy
```

### Commit Messages

Follow conventional commits:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `test:` Tests
- `refactor:` Code refactoring

## Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# Coverage
cargo tarpaulin
```

## Debugging

```bash
# Debug build
cargo build --debug

# Run with logging
RUST_LOG=debug cargo run

# Use lldb
rust-lldb ./target/debug/vantis
```

## Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Run tests
5. Submit pull request

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.