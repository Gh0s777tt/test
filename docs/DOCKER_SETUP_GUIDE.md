# VantisOS Docker Development Setup Guide

## Overview

This guide explains how to set up and use Docker for VantisOS development, providing a consistent and isolated development environment.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Quick Start](#quick-start)
3. [Docker Setup](#docker-setup)
4. [Docker Compose](#docker-compose)
5. [Dev Containers](#dev-containers)
6. [Common Operations](#common-operations)
7. [Troubleshooting](#troubleshooting)
8. [Best Practices](#best-practices)

## Prerequisites

### Required Software

- **Docker**: https://docs.docker.com/get-docker/
- **Docker Compose**: Included with Docker Desktop
- **Git**: https://git-scm.com/

### System Requirements

- **RAM**: 8GB minimum (16GB recommended)
- **CPU**: 4 cores minimum (8 cores recommended)
- **Disk Space**: 20GB minimum (50GB recommended)

### Verify Installation

```bash
docker --version
docker-compose --version
```

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS
```

### 2. Build the Development Container

```bash
cd docker
docker build -f Dockerfile.dev -t vantisos-dev:latest .
```

### 3. Run the Development Container

```bash
docker run -it --rm \
  -v $(pwd)/..:/workspace \
  -v cargo-cache:/usr/local/cargo \
  --cap-add=SYS_ADMIN \
  --device=/dev/kvm:/dev/kvm \
  vantisos-dev:latest
```

### 4. Build the Project Inside Container

```bash
cargo build
cargo test
```

## Docker Setup

### Building the Development Image

Build the development Docker image:

```bash
docker build -f docker/Dockerfile.dev -t vantisos-dev:latest docker/
```

Build with build arguments for custom user:

```bash
docker build -f docker/Dockerfile.dev \
  --build-arg USERNAME=myuser \
  --build-arg USER_UID=1000 \
  -t vantisos-dev:latest \
  docker/
```

### Running the Container

#### Interactive Shell

```bash
docker run -it --rm \
  -v $(pwd):/workspace \
  vantisos-dev:latest
```

#### With Docker Compose

```bash
cd docker
docker-compose run --rm dev
```

#### With VS Code Dev Containers

Open in VS Code:
1. Install "Remote - Containers" extension
2. Press `F1` → "Dev Containers: Open Folder in Container"
3. Select the VantisOS folder

## Docker Compose

### Starting Services

Start the development environment:

```bash
cd docker
docker-compose up -d
```

Start with additional services:

```bash
# With PostgreSQL
docker-compose --profile postgres up -d

# With Redis
docker-compose --profile redis up -d

# With all services
docker-compose --profile postgres --profile redis up -d
```

### Stopping Services

Stop all services:

```bash
docker-compose down
```

Stop with volumes:

```bash
docker-compose down -v
```

### Viewing Logs

View logs from all services:

```bash
docker-compose logs -f
```

View logs from specific service:

```bash
docker-compose logs -f dev
```

### Service Profiles

Available profiles:
- **default**: Development environment only
- **postgres**: Includes PostgreSQL database
- **redis**: Includes Redis cache
- **docs**: Documentation server
- **metrics**: Prometheus metrics

## Dev Containers

### Opening in Dev Container

#### Using VS Code

1. Install "Remote - Containers" extension
2. Open VantisOS repository in VS Code
3. Press `F1` → "Dev Containers: Reopen in Container"
4. VS Code will build and connect to the container

#### Using Command Line

```bash
# Install devcontainer CLI
npm install -g @devcontainers/cli

# Open repository in container
devcontainer open /path/to/VantisOS
```

### Container Features

The dev container includes:

- **Rust Toolchain**: Latest stable Rust
- **Cargo Tools**: watch, expand, edit, audit, outdated
- **QEMU**: For kernel testing
- **Development Tools**: git, vim, nano, htop, gdb, lldb
- **Python**: For tooling and scripts
- **Node.js**: For web development tools
- **VS Code Extensions**: Pre-installed extensions

### Container Customization

#### Modify Configuration

Edit `.devcontainer/devcontainer.json`:

```json
{
  "customizations": {
    "vscode": {
      "extensions": [
        // Add your extensions here
      ]
    }
  }
}
```

#### Rebuild Container

```bash
# From VS Code
F1 → "Dev Containers: Rebuild Container"

# From command line
devcontainer build --workspace-folder .
devcontainer up --workspace-folder .
```

## Common Operations

### Building the Project

```bash
cargo build
cargo build --release
cargo build --bin vantis-kernel
```

### Running Tests

```bash
# All tests
cargo test

# Specific package
cargo test -p vantis-kernel

# With output
cargo test -- --nocapture

# Run in release mode
cargo test --release
```

### Running with QEMU

```bash
# Build kernel
cargo build --bin vantis-kernel

# Run in QEMU
qemu-system-x86_64 \
  -kernel target/x86_64-vantis-kernel/debug/vantis-kernel \
  -m 512M \
  -serial stdio \
  -display none
```

### Code Quality Checks

```bash
# Format code
cargo fmt

# Check code
cargo check

# Run linter
cargo clippy

# All checks
cargo fmt && cargo check && cargo clippy && cargo test
```

### Documentation

```bash
# Generate documentation
cargo doc --open

# Build MkDocs
cd docs
mkdocs build

# Serve documentation
mkdocs serve
```

### Repository Health Check

```bash
./scripts/health_check.sh --verbose
```

### Cleaning Up

```bash
# Clean build artifacts
cargo clean

# Clean Docker images
docker system prune -a

# Clean Docker volumes
docker volume prune

# Clean everything
docker system prune -a --volumes
```

## Troubleshooting

### Docker Issues

#### Issue: Permission Denied

**Solution**: Add user to docker group
```bash
sudo usermod -aG docker $USER
newgrp docker
```

#### Issue: Container Won't Start

**Solution**: Check logs
```bash
docker-compose logs dev
docker logs vantisos-dev
```

#### Issue: Build Fails

**Solution**: Clear cache and rebuild
```bash
docker builder prune
docker-compose build --no-cache
```

#### Issue: Out of Disk Space

**Solution**: Clean Docker
```bash
docker system df
docker system prune -a
```

### Performance Issues

#### Slow Builds

**Solution**: Enable cargo caching
```yaml
volumes:
  cargo-cache:
    driver: local
```

#### High Memory Usage

**Solution**: Limit resources
```yaml
deploy:
  resources:
    limits:
      memory: 4G
```

### QEMU Issues

#### QEMU Not Found

**Solution**: Install QEMU
```bash
docker exec vantisos-dev apt-get update
docker exec vantisos-dev apt-get install -y qemu-system-x86
```

#### Permission Denied on /dev/kvm

**Solution**: Add capabilities
```bash
docker run --cap-add=SYS_ADMIN --device=/dev/kvm:/dev/kvm ...
```

### VS Code Dev Container Issues

#### Container Won't Open

**Solution**: Rebuild container
```bash
F1 → "Dev Containers: Rebuild Container"
```

#### Extensions Not Installing

**Solution**: Check internet connection and rebuild
```bash
F1 → "Dev Containers: Rebuild Container"
```

#### Files Not Syncing

**Solution**: Check volume mounts
```json
{
  "workspaceMount": "source=${localWorkspaceFolder},target=/workspace,type=bind"
}
```

## Best Practices

### Development Workflow

1. **Use Dev Containers** for consistent environment
2. **Leverage Caching** for faster builds
3. **Run Tests Frequently** to catch issues early
4. **Use Compose** for managing services
5. **Clean Up** unused resources regularly

### Performance Optimization

1. **Use Cached Volumes**
   ```yaml
   volumes:
     cargo-cache: # Reuse cargo cache
   ```

2. **Limit Resources**
   ```yaml
   deploy:
     resources:
       limits:
         cpus: '4'
         memory: 8G
   ```

3. **Use BuildKit**
   ```bash
   export DOCKER_BUILDKIT=1
   docker build ...
   ```

4. **Multi-stage Builds**
   - Use builder stage for dependencies
   - Copy only necessary artifacts

### Security

1. **Don't Run as Root**
   ```dockerfile
   USER vantis
   ```

2. **Minimal Privileges**
   ```yaml
   cap_add:
     - SYS_ADMIN
   ```

3. **Update Base Images**
   ```bash
   docker pull rust:latest
   ```

4. **Scan for Vulnerabilities**
   ```bash
   docker scan vantisos-dev:latest
   ```

### Collaboration

1. **Share Configuration**
   - Commit Dockerfile
   - Commit docker-compose.yml
   - Commit devcontainer.json

2. **Document Dependencies**
   - Document required services
   - Document ports
   - Document volumes

3. **Use Version Tags**
   ```bash
   docker build -t vantisos-dev:v1.0.0 ...
   ```

## Advanced Usage

### Custom Dockerfile

Create custom Dockerfile for specific needs:

```dockerfile
FROM vantisos-dev:latest

# Install additional tools
RUN apt-get update && apt-get install -y \
    your-tool \
    && rm -rf /var/lib/apt/lists/*
```

### Multi-stage Builds

Use multi-stage builds for optimization:

```dockerfile
FROM rust:1.75 AS builder
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /target/release/vantis-kernel /usr/local/bin/
```

### Docker Compose Overrides

Create `docker-compose.override.yml` for local customization:

```yaml
services:
  dev:
    ports:
      - "3000:3000"  # Add custom port
    volumes:
      - ./local:/workspace/local  # Add local volume
```

### Health Checks

Add health checks to services:

```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
  interval: 30s
  timeout: 10s
  retries: 3
```

## Resources

### Documentation

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Dev Containers Documentation](https://code.visualstudio.com/docs/devcontainers/containers)

### Tools

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [VS Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers)
- [Docker Hub](https://hub.docker.com/)

### Community

- [Docker Community](https://www.docker.com/community)
- [VantisOS Issues](https://github.com/vantisCorp/VantisOS/issues)

## Conclusion

Using Docker for VantisOS development provides:

- **Consistent Environment**: Same environment across all machines
- **Isolation**: Clean separation from host system
- **Portability**: Easy to share and reproduce
- **Flexibility**: Easy to customize and extend
- **Scalability**: Easy to scale services with Docker Compose

For additional help or questions, see:
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)
- [VantisOS Documentation](https://github.com/vantisCorp/VantisOS)

---

**Version**: 1.0.0  
**Last Updated**: March 2025  
**Maintained by**: VantisOS Team