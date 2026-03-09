# 🚀 VantisOS Quick Start Guide

Welcome to VantisOS - A formally verified, mathematically proven operating system built with Rust and Verus.

---

## 📋 Prerequisites

Before you begin, make sure you have:

- **Rust 1.70+**: [Install Rust](https://www.rust-lang.org/tools/install)
- **QEMU 6.0+**: [Install QEMU](https://www.qemu.org/download/)
- **Git**: [Install Git](https://git-scm.com/downloads)
- **Linux**: Ubuntu 20.04+, Debian 11+, or compatible distribution

---

## 🎯 Quick Start (5 minutes)

### 1. Clone the Repository

```bash
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS
```

### 2. Build the Kernel

```bash
# Build everything
./scripts/build_all.sh

# Or build specific components
./scripts/build_kernel.sh    # Build only kernel
./scripts/build_iso.sh       # Create bootable ISO
```

### 3. Run in QEMU

```bash
# Run VantisOS in QEMU
./scripts/run_qemu.sh
```

**That's it!** You should now see VantisOS boot in a QEMU window.

---

## 🐳 Cloud Native Quick Start

VantisOS v1.4.0 includes comprehensive AI features, advanced security, and cloud-native capabilities:

### 1. Set up Cloud Provider Credentials

```bash
# AWS
export AWS_ACCESS_KEY_ID="your_access_key"
export AWS_SECRET_ACCESS_KEY="your_secret_key"
export AWS_DEFAULT_REGION="us-east-1"

# Azure
export AZURE_SUBSCRIPTION_ID="your_subscription_id"
export AZURE_CLIENT_ID="your_client_id"
export AZURE_CLIENT_SECRET="your_client_secret"
export AZURE_TENANT_ID="your_tenant_id"

# GCP
export GCP_PROJECT_ID="your_project_id"
export GCP_SERVICE_ACCOUNT_KEY="path/to/key.json"
```

### 2. Deploy to Kubernetes

```bash
# See examples/kubernetes_basic.rs for more details
cargo run --example kubernetes_basic
```

### 3. Multi-Cloud Deployment

```bash
# See examples/multi_cloud_deploy.rs for more details
cargo run --example multi_cloud_deploy
```

---

## 🧪 Testing

```bash
# Run all tests
./scripts/test_all.sh

# Run specific test suites
cargo test --lib                    # Unit tests
cargo test --test integration       # Integration tests
cargo test --test integration_cloud_native  # Cloud-native tests
```

---

## 📚 Learning Resources

### Documentation

- **[README.md](README.md)** - Complete project overview
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and changes
- **[RELEASE_NOTES.md](RELEASE_NOTES.md)** - Latest release notes
- **[TODO.md](TODO.md)** - Current development roadmap
- **[ROADMAP.md](ROADMAP.md)** - Future development plans

### Specialized Guides

- **[CLOUD_NATIVE_GUIDE.md](docs/CLOUD_NATIVE_GUIDE.md)** - Cloud-native features
- **[MIGRATION_GUIDE_v1.4.0.md](docs/MIGRATION_GUIDE_v1.4.0.md)** - Migration from v1.3.1
- **[DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md)** - Developer documentation
- **[API_REFERENCE.md](API_REFERENCE.md)** - Complete API documentation

### Examples

- **`examples/kubernetes_basic.rs`** - Basic Kubernetes operations
- **`examples/multi_cloud_deploy.rs`** - Multi-cloud deployment
- **`examples/distributed_systems.rs`** - Distributed computing

---

## 🛠️ Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

```bash
# Edit files
nano src/verified/your_module.rs

# Build and test
./scripts/build_all.sh
./scripts/test_all.sh
```

### 3. Commit Your Changes

```bash
git add .
git commit -m "feat: Add your feature description"
```

### 4. Push and Create PR

```bash
git push origin feature/your-feature-name
# Then create a Pull Request on GitHub
```

---

## 🐛 Troubleshooting

### Build Errors

**Problem**: Build fails with "error: toolchain is not installed"

**Solution**:
```bash
rustup install stable
rustup default stable
```

**Problem**: Missing dependencies

**Solution**:
```bash
sudo apt-get install build-essential qemu-system-x86 grub2-common xorriso
```

### QEMU Issues

**Problem**: QEMU won't start

**Solution**:
```bash
# Check if QEMU is installed
qemu-system-x86_64 --version

# Install QEMU if not present
sudo apt-get install qemu-system-x86
```

### Cloud Native Issues

**Problem**: Authentication errors

**Solution**:
- Verify your cloud provider credentials
- Check IAM permissions
- Ensure service account has correct roles

---

## 📊 Getting Help

### Community

- **GitHub Issues**: [Report bugs or request features](https://github.com/vantisCorp/VantisOS/issues)
- **Discord**: [Join the VantisOS Citadel](https://discord.gg/dSxQXXVBhx)
- **Documentation**: [Full documentation](docs/)

### Additional Resources

- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines
- **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)** - Code of conduct
- **[SECURITY.md](SECURITY.md)** - Security policies

---

## 🎉 Next Steps

1. **Explore Features**: Read the [README.md](README.md) to learn about all features
2. **Run Examples**: Try the examples in the `examples/` directory
3. **Contribute**: Check [TODO.md](TODO.md) for open tasks
4. **Read Documentation**: Dive deeper into specialized guides

---

## 📈 Metrics

- **Version**: v1.4.0 "AI Advanced Features"
- **Lines of Code**: ~141,000+
- **Test Coverage**: 65% (800+ tests)
- **Supported Architectures**: x86_64, ARM64, RISC-V
- **Cloud Providers**: AWS, Azure, GCP
- **Certifications**: ISO 27001:2022, SOC 2 Type II, PCI DSS, HIPAA, FIPS 140-3, EAL 7+

---

**Happy Hacking! 🚀**

*Last Updated: March 3, 2026*