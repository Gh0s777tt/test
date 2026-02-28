# 🤝 Contributing to VANTIS OS - Contribution Guide

[![Contributors](https://img.shields.io/badge/contributors-0-blue?style=for-the-badge)](https://github.com/vantisCorp/VantisOS/graphs/contributors) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=for-the-badge)](http://makeapullrequest.com) [![License](https://img.shields.io/badge/license-MIT-purple?style=for-the-badge)](LICENSE)

**✨ Thank you for your interest in contributing to VANTIS OS! ✨**

## 📊 Project Status (February 28, 2025)

**VantisOS 0.4.1 "Cytadela Complete" is now 100% complete with all 18 priorities implemented!**

- ✅ All 18 priorities complete (100%)
- ✅ 50,000+ lines of code
- ✅ 40,000+ lines of documentation
- ✅ 209 Rust files
- ✅ 394 tests with 60% coverage
- ✅ 7+ certifications (100% compliance)
- ✅ Redox OS bootloader integrated
- ✅ Auto-boot feature implemented
- ✅ Production ready

## 📋 Table of Contents

-   [🎯 Code of Conduct](#-code-of-conduct)
-   [🚀 Getting Started](#-getting-started)
-   [🔧 Development Process](#-development-process)
-   [📝 Code Standards](#-code-standards)
-   [🧪 Testing](#-testing)
-   [🤝 Pull Requests](#-pull-requests)
-   [🐛 Bug Reports](#-bug-reports)
-   [💡 Feature Requests](#-feature-requests)
-   [📚 Documentation](#-documentation)

---

## 🎯 Code of Conduct

### Our Pledge

In the interest of fostering an open and welcoming environment, we as contributors and maintainers pledge to making participation in our project and our community a harassment-free experience for everyone.

### Our Standards

Examples of behavior that contributes to creating a positive environment include:

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

### Our Responsibilities

Project maintainers are responsible for clarifying the standards of acceptable behavior and are expected to take appropriate and fair corrective action in response to any instances of unacceptable behavior.

### Scope

This Code of Conduct applies both within project spaces and in public spaces when an individual is representing the project or its community.

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team at conduct@vantis.os. All complaints will be reviewed and investigated and will result in a response that is deemed necessary and appropriate to the circumstances.

---

## 🚀 Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/VantisOS.git
cd VantisOS

# Add upstream
git remote add upstream https://github.com/vantisCorp/VantisOS.git
```

### 2. Set Up Development Environment

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
cargo install cargo-edit
cargo install cargo-watch

# Build the project
cargo build --release
```

### 3. Run Tests

```bash
# Run all tests
cargo test --all

# Run tests with coverage
cargo tarpaulin --out Html

# Run specific test suite
cargo test --test unit_tests
```

---

## 🔧 Development Process

### Branching Strategy

We use a simplified branching strategy:

- **0.4.1** - Main development branch (default)
- **feature/*** - Feature branches
- **fix/*** - Bug fix branches
- **docs/*** - Documentation branches

### Workflow

1. Create a new branch from `0.4.1`
2. Make your changes
3. Write tests for your changes
4. Ensure all tests pass
5. Update documentation if needed
6. Create a pull request

### Commit Messages

Follow conventional commits format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(audio): add Dolby Atmos 7.1.4 support
fix(iommu): resolve data race in global IOMMU
docs(readme): update project statistics
```

---

## 📝 Code Standards

### Rust Style Guide

We follow the official Rust style guide:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

### Naming Conventions

- **Structs**: `PascalCase` - `struct VantisKernel {}`
- **Functions**: `snake_case` - `fn initialize_kernel() {}`
- **Constants**: `SCREAMING_SNAKE_CASE` - `const MAX_THREADS: usize = 16;`
- **Modules**: `snake_case` - `mod kernel_allocator {}`

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use `Option<T>` for optional values
- Provide meaningful error messages
- Use `thiserror` for custom error types

Example:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KernelError {
    #[error("Memory allocation failed")]
    AllocationFailed,
    
    #[error("Invalid address: {0}")]
    InvalidAddress(usize),
}

pub fn allocate_memory(size: usize) -> Result<*mut u8, KernelError> {
    // implementation
}
```

### Documentation

Document all public APIs:

```rust
/// Allocates a block of memory of the specified size.
///
/// # Arguments
///
/// * `size` - The size of the memory block to allocate in bytes
///
/// # Returns
///
/// * `Ok(*mut u8)` - Pointer to the allocated memory
/// * `Err(KernelError)` - If allocation fails
///
/// # Examples
///
/// ```
/// let ptr = allocate_memory(1024)?;
/// ```
pub fn allocate_memory(size: usize) -> Result<*mut u8, KernelError> {
    // implementation
}
```

---

## 🧪 Testing

### Test Structure

We have three levels of tests:

1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test interactions between modules
3. **Property-Based Tests** - Test invariants with random inputs

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_allocation() {
        let ptr = allocate_memory(1024).unwrap();
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_invalid_size() {
        let result = allocate_memory(0);
        assert!(matches!(result, Err(KernelError::AllocationFailed)));
    }
}
```

### Test Coverage

We aim for 60%+ test coverage. Current coverage: **60%** (394 tests)

Run coverage report:
```bash
./tests/generate_coverage.sh
```

---

## 🤝 Pull Requests

### Before Submitting

- [ ] Code follows style guidelines (`cargo fmt`, `cargo clippy`)
- [ ] All tests pass (`cargo test --all`)
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] Commit messages follow conventional commits format

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All tests pass

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No merge conflicts
```

### Review Process

1. Automated checks must pass
2. At least one maintainer approval required
3. Address all review comments
4. Squash and merge to `0.4.1`

---

## 🐛 Bug Reports

### Bug Report Template

```markdown
## Description
Clear and concise description of the bug

## Steps to Reproduce
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

## Expected Behavior
What you expected to happen

## Actual Behavior
What actually happened

## Environment
- OS: [e.g., Linux, Windows]
- Version: [e.g., 0.4.1]
- Rust version: [e.g., 1.93.1]

## Additional Context
Add any other context about the problem here
```

---

## 💡 Feature Requests

### Feature Request Template

```markdown
## Description
Clear and concise description of the feature

## Motivation
Why is this feature needed?

## Proposed Solution
How should this feature work?

## Alternatives
What alternatives have you considered?

## Additional Context
Add any other context about the feature request here
```

---

## 📚 Documentation

### Documentation Structure

- **README.md** - Main project documentation
- **docs/api/** - API documentation
- **docs/developer/** - Developer guides
- **docs/testing/** - Testing documentation
- **docs/compliance/** - Compliance documentation
- **DOCUMENTATION_INDEX.md** - Complete documentation index

### Writing Documentation

- Use clear, concise language
- Include code examples
- Add diagrams where helpful
- Keep documentation up to date
- Use Markdown formatting

---

## 🎉 Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- Project documentation

Thank you for contributing to VantisOS! 🚀

---

## 📞 Contact

- **GitHub Issues**: [github.com/vantisCorp/VantisOS/issues](https://github.com/vantisCorp/VantisOS/issues)
- **Discord**: [Join our Discord](https://discord.gg/dSxQXXVBhx)
- **Email**: [info@vantis.os](mailto:info@vantis.os)