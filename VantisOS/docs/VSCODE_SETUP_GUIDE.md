# VantisOS VS Code Development Setup Guide

## Overview

This guide explains how to set up Visual Studio Code for optimal VantisOS development, including configuration, extensions, debugging, and workflows.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [VS Code Configuration](#vs-code-configuration)
4. [Recommended Extensions](#recommended-extensions)
5. [Debug Configuration](#debug-configuration)
6. [Tasks and Automation](#tasks-and-automation)
7. [Keyboard Shortcuts](#keyboard-shortcuts)
8. [Workflows](#workflows)
9. [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Tools

Before setting up VS Code, ensure you have:

- **VS Code** (latest version): https://code.visualstudio.com/
- **Rust Toolchain**: https://rustup.rs/
- **Cargo**: Installed with Rust
- **Git**: https://git-scm.com/

### Optional Tools

- **QEMU**: For kernel testing
- **LLDB**: For debugging (included with Rust)
- **Docker**: For containerized development

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS
```

### 2. Open in VS Code

```bash
code .
```

Or open VS Code and use `File > Open Folder...` to select the VantisOS directory.

### 3. Install Recommended Extensions

When you open the project for the first time, VS Code will prompt you to install recommended extensions. Click **Install All**.

Alternatively, install manually:

```bash
# Install all recommended extensions at once
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension cratesio.cargo
code --install-extension tamasfe.even-better-toml
code --install-extension swellaby.vscode-rust-test-adapter
# ... and others from .vscode/extensions.json
```

## VS Code Configuration

The `.vscode/settings.json` file contains optimized settings for VantisOS development:

### Key Features

1. **Auto-formatting on Save**
   ```json
   "editor.formatOnSave": true
   ```

2. **Rust Analyzer Configuration**
   - Clippy integration
   - Inlay hints enabled
   - Enhanced completions
   - Semantic highlighting

3. **Code Actions on Save**
   ```json
   "editor.codeActionsOnSave": {
     "source.fixAll": "explicit",
     "source.organizeImports": "explicit"
   }
   ```

4. **Custom Snippets**
   - Rust test templates
   - Benchmark templates
   - Common patterns

### Customizing Settings

To override project settings, create a workspace-specific settings file:

1. Open Settings (`Ctrl+,` or `Cmd+,`)
2. Select **Workspace** tab
3. Modify settings as needed

## Recommended Extensions

### Core Rust Development

| Extension | Purpose |
|-----------|---------|
| `rust-lang.rust-analyzer` | Rust language server |
| `vadimcn.vscode-lldb` | Debugging support |
| `cratesio.cargo` | Crates.io integration |
| `tamasfe.even-better-toml` | TOML formatting |
| `swellaby.vscode-rust-test-adapter` | Test runner |

### Formal Verification

| Extension | Purpose |
|-----------|---------|
| `github.copilot` | AI assistance |
| `github.copilot-chat` | AI chat assistant |

### Code Quality

| Extension | Purpose |
|-----------|---------|
| `esbenp.prettier-vscode` | Code formatting |
| `editorconfig.editorconfig` | Editor configuration |
| `DavidAnson.vscode-markdownlint` | Markdown linting |
| `timonwong.shellcheck` | Shell script linting |

### Git & Version Control

| Extension | Purpose |
|-----------|---------|
| `eamodio.gitlens` | Git superpowers |
| `github.vscode-pull-request-github` | PR management |
| `github.vscode-github-actions` | Workflow management |

## Debug Configuration

The `.vscode/launch.json` file provides multiple debug configurations:

### Available Configurations

1. **Debug VantisOS Kernel**
   - Builds and runs the kernel
   - Enabled debugging symbols
   - Logs set to debug level

2. **Debug Unit Tests**
   - Runs unit tests in debug mode
   - Break on test failures

3. **Debug Integration Tests**
   - Runs integration tests
   - Full debugging support

4. **Debug with QEMU**
   - Launches kernel in QEMU
   - Attached to debugger
   - Full kernel debugging

### Using the Debugger

1. Set breakpoints in your code (click in the gutter)
2. Select a configuration from the Debug panel (`Ctrl+Shift+D`)
3. Press `F5` to start debugging
4. Use debug controls:
   - `F5` - Continue
   - `F10` - Step over
   - `F11` - Step into
   - `Shift+F11` - Step out
   - `Shift+F5` - Stop debugging

### Debugging Tips

- **Conditional Breakpoints**: Right-click breakpoint → Edit Breakpoint
- **Logpoints**: Right-click breakpoint → Add Logpoint
- **Watch Variables**: Add variables to Watch panel
- **Call Stack**: Navigate the call stack in the left panel
- **Evaluate Expressions**: Use the Debug Console

## Tasks and Automation

The `.vscode/tasks.json` file defines common development tasks:

### Available Tasks

#### Build Tasks
- `cargo: build` - Build the workspace (default)
- `cargo: build vantis-kernel` - Build kernel only
- `cargo: check` - Check without building
- `make: build` - Use Makefile build target

#### Test Tasks
- `cargo: test` - Run all tests (default)
- `cargo: test vantis-kernel` - Run kernel tests
- `make: test` - Use Makefile test target

#### Quality Tasks
- `cargo: clippy` - Run linter
- `cargo: fmt` - Format code
- `make: all` - Run all checks

#### QEMU Tasks
- `qemu: run kernel` - Run kernel in QEMU (no display)
- `qemu: run kernel with display` - Run kernel with display

#### Utility Tasks
- `scripts: health check` - Run repository health check
- `cargo: doc` - Generate and open documentation

### Running Tasks

1. **Command Palette**: `Ctrl+Shift+P` → "Tasks: Run Task"
2. **Terminal**: Click the gear icon → Run Task
3. **Keyboard**: Configure shortcuts in `keybindings.json`

### Building Tasks

To build before running tests or debugging:

```json
{
  "label": "Debug VantisOS Kernel",
  "preLaunchTask": "cargo: build vantis-kernel"
}
```

## Keyboard Shortcuts

### Default VS Code Shortcuts (Windows/Linux)

| Action | Shortcut |
|--------|----------|
| Command Palette | `Ctrl+Shift+P` |
| Quick Open | `Ctrl+P` |
| File Explorer | `Ctrl+Shift+E` |
| Search | `Ctrl+Shift+F` |
| Source Control | `Ctrl+Shift+G` |
| Debug | `Ctrl+Shift+D` |
| Extensions | `Ctrl+Shift+X` |
| Terminal | `Ctrl+\`` |
| Build | `Ctrl+Shift+B` |
| Run Task | `Ctrl+Shift+P` → "Tasks: Run Task" |

### Rust-Specific Shortcuts

| Action | Shortcut |
|--------|----------|
| Go to Definition | `F12` |
| Peek Definition | `Alt+F12` |
| Find References | `Shift+F12` |
| Rename Symbol | `F2` |
| Format Document | `Shift+Alt+F` |
| Go to Problems | `Ctrl+Shift+M` |

### Custom Shortcuts

Add custom shortcuts in `.vscode/keybindings.json`:

```json
[
  {
    "key": "ctrl+shift+t",
    "command": "workbench.action.terminal.new",
    "when": "!terminalFocus"
  },
  {
    "key": "ctrl+shift+r",
    "command": "workbench.action.tasks.runTask",
    "args": "cargo: test"
  }
]
```

## Workflows

### Daily Development Workflow

1. **Start**: Open VS Code
   ```bash
   code .
   ```

2. **Pull Latest Changes**
   - Use GitLens: Sync Changes → Pull
   - Or use GitLens graph view

3. **Build the Project**
   - `Ctrl+Shift+B` (Build task)
   - Or `cargo build` in terminal

4. **Run Tests**
   - `Ctrl+Shift+P` → "Tasks: Run Task" → `cargo: test`
   - Or use the test explorer

5. **Develop Features**
   - Write code with IntelliSense
   - Use Copilot for suggestions
   - Format on save

6. **Debug Issues**
   - Set breakpoints
   - Run with `F5`
   - Inspect variables

7. **Commit Changes**
   - Stage files in GitLens
   - Write commit message
   - Push changes

### Testing Workflow

1. **Unit Tests**
   - Write test in `src/`
   - Run with `cargo test`
   - Debug failing tests

2. **Integration Tests**
   - Create test file in `tests/`
   - Run with `cargo test --test integration_test`
   - Debug with integration test config

3. **Kernel Testing**
   - Build kernel: `cargo: build vantis-kernel`
   - Run in QEMU: `qemu: run kernel`
   - Debug with QEMU config

### Code Review Workflow

1. **Review PR**
   - Open PR in VS Code
   - Use GitHub PR extension
   - Comment inline
   - Request changes

2. **Local Review**
   - Checkout branch
   - Build and test
   - Leave comments

## Troubleshooting

### Common Issues

#### Issue: Rust Analyzer Not Working

**Solution:**
1. Check Rust is installed: `rustc --version`
2. Restart Rust Analyzer: `Ctrl+Shift+P` → "Rust Analyzer: Restart Server"
3. Update Rust: `rustup update`

#### Issue: Debugger Not Launching

**Solution:**
1. Install LLDB: `rustup component add llvm-tools-preview`
2. Check launch.json configuration
3. Verify binary exists in `target/`

#### Issue: Extensions Not Loading

**Solution:**
1. Check internet connection
2. Reload VS Code: `Ctrl+Shift+P` → "Developer: Reload Window"
3. Check for extension updates

#### Issue: Auto-formatting Not Working

**Solution:**
1. Check formatter is installed: `rustfmt --version`
2. Verify settings: `"editor.formatOnSave": true`
3. Check language mode is "Rust"

#### Issue: QEMU Not Found

**Solution:**
1. Install QEMU: `sudo apt-get install qemu-system-x86`
2. Verify installation: `qemu-system-x86_64 --version`
3. Update PATH if needed

### Performance Tips

1. **Disable Unnecessary Extensions**
   - Open Extensions panel
   - Disable extensions not in use

2. **Exclude Large Directories**
   - Settings → Files → Exclude
   - Add `target`, `.history`

3. **Increase Memory Limit**
   - Settings → Features → Performance
   - Adjust workspace memory

4. **Use Workspace Trust**
   - Settings → Workspace → Trust
   - Trust the VantisOS workspace

### Getting Help

1. **VS Code Help**: `F1`
2. **Rust Analyzer Help**: `Ctrl+Shift+P` → "Rust Analyzer: Open Docs"
3. **Extension Documentation**: Extension details → Documentation link
4. **VantisOS Issues**: https://github.com/vantisCorp/VantisOS/issues

## Advanced Features

### Remote Development

Use VS Code Remote for containerized development:

```bash
# Install Remote SSH extension
code --install-extension ms-vscode-remote.remote-ssh

# Connect to remote
# Press F1 → "Remote-SSH: Connect to Host"
```

### Multi-root Workspace

Add multiple projects to a workspace:

1. **File → Add Folder to Workspace**
2. Add `src/`, `tests/`, etc.
3. Each folder has its own `.vscode` config

### Dev Containers

Use the included dev container configuration:

```bash
# Install Remote Containers extension
code --install-extension ms-vscode-remote.remote-containers

# Open in container
# Press F1 → "Dev Containers: Reopen in Container"
```

### Profile Customization

Create profiles for different workflows:

1. **File → Preferences → Profiles → Create Profile**
2. Configure settings per profile
3. Switch profiles as needed

## Best Practices

1. **Keep Extensions Updated**
   - Auto-update enabled
   - Review updates weekly

2. **Use Workspaces**
   - One workspace per project
   - Share workspace settings with team

3. **Customize Keyboard Shortcuts**
   - Adapt to your workflow
   - Document custom shortcuts

4. **Use Integrated Terminal**
   - Run cargo commands
   - Execute scripts
   - Git operations

5. **Leverage GitLens**
   - Blame annotations
   - Commit history
   - PR management

## Conclusion

This setup provides an optimal development environment for VantisOS with:

- **Intelligent Code Editing**: Rust Analyzer
- **Powerful Debugging**: LLDB integration
- **Automated Workflows**: Tasks and builds
- **Enhanced Git**: GitLens integration
- **AI Assistance**: GitHub Copilot

For more information, see:
- [VS Code Documentation](https://code.visualstudio.com/docs)
- [Rust Analyzer Documentation](https://rust-analyzer.github.io/)
- [VantisOS Documentation](https://github.com/vantisCorp/VantisOS)

---

**Version**: 1.0.0  
**Last Updated**: March 2025  
**Maintained by**: VantisOS Team