#!/bin/bash
# VantisOS Full Build - Automated Setup Script
# This script automates the initial setup for building VantisOS

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REDOX_REPO="https://gitlab.redox-os.org/redox-os/redox.git"
WORK_DIR="$HOME/vantis-build"
REDOX_DIR="$WORK_DIR/redox-base"
VANTIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Functions
print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

check_dependencies() {
    print_header "Checking Dependencies"
    
    local missing_deps=()
    
    # Check for required tools
    for cmd in git curl rustc cargo qemu-system-x86_64 nasm make; do
        if ! command -v $cmd &> /dev/null; then
            missing_deps+=($cmd)
        else
            print_success "$cmd found"
        fi
    done
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing dependencies: ${missing_deps[*]}"
        echo ""
        print_info "Install missing dependencies:"
        echo "  Ubuntu/Debian: sudo apt-get install git curl build-essential nasm qemu-system-x86 cargo rustc"
        echo "  Arch Linux: sudo pacman -S git curl base-devel nasm qemu rust"
        echo "  Fedora: sudo dnf install git curl @development-tools nasm qemu rust cargo"
        exit 1
    fi
    
    print_success "All dependencies found!"
}

check_disk_space() {
    print_header "Checking Disk Space"
    
    local available=$(df -BG "$HOME" | awk 'NR==2 {print $4}' | sed 's/G//')
    local required=50
    
    if [ "$available" -lt "$required" ]; then
        print_error "Insufficient disk space. Required: ${required}GB, Available: ${available}GB"
        exit 1
    fi
    
    print_success "Sufficient disk space: ${available}GB available"
}

setup_work_directory() {
    print_header "Setting Up Work Directory"
    
    if [ -d "$WORK_DIR" ]; then
        print_warning "Work directory already exists: $WORK_DIR"
        read -p "Remove and recreate? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -rf "$WORK_DIR"
            print_success "Removed existing work directory"
        else
            print_info "Using existing work directory"
            return
        fi
    fi
    
    mkdir -p "$WORK_DIR"
    print_success "Created work directory: $WORK_DIR"
}

clone_redox() {
    print_header "Cloning Redox OS"
    
    if [ -d "$REDOX_DIR" ]; then
        print_warning "Redox already cloned"
        return
    fi
    
    print_info "Cloning Redox OS (this may take a while)..."
    git clone --recursive "$REDOX_REPO" "$REDOX_DIR"
    print_success "Redox OS cloned successfully"
}

install_redox_dependencies() {
    print_header "Installing Redox Dependencies"
    
    cd "$REDOX_DIR"
    
    print_info "Running Redox bootstrap script..."
    ./bootstrap.sh -d
    
    print_success "Redox dependencies installed"
}

integrate_vantis_components() {
    print_header "Integrating VantisOS Components"
    
    # Create VantisOS directory in Redox kernel
    local vantis_kernel_dir="$REDOX_DIR/kernel/src/vantis"
    mkdir -p "$vantis_kernel_dir"
    
    # Copy VantisOS verified components
    print_info "Copying VantisOS components..."
    cp -r "$VANTIS_DIR/src/verified/"* "$vantis_kernel_dir/"
    print_success "VantisOS components copied"
    
    # Update kernel Cargo.toml
    print_info "Updating kernel Cargo.toml..."
    if ! grep -q "vantis-verified" "$REDOX_DIR/kernel/Cargo.toml"; then
        cat >> "$REDOX_DIR/kernel/Cargo.toml" << 'EOF'

# VantisOS Components
[dependencies.vantis-verified]
path = "src/vantis"
EOF
        print_success "Kernel Cargo.toml updated"
    else
        print_warning "Kernel Cargo.toml already contains VantisOS entry"
    fi
}

create_vantis_config() {
    print_header "Creating VantisOS Configuration"
    
    local config_file="$REDOX_DIR/config/x86_64/vantis.toml"
    
    if [ -f "$config_file" ]; then
        print_warning "VantisOS config already exists"
        return
    fi
    
    # Copy desktop config as base
    cp "$REDOX_DIR/config/x86_64/desktop.toml" "$config_file"
    
    # Customize for VantisOS
    sed -i 's/name = "desktop"/name = "vantis"/' "$config_file"
    sed -i 's/prompt = ".*"/prompt = "vantis"/' "$config_file"
    
    print_success "VantisOS configuration created: $config_file"
}

test_build() {
    print_header "Testing Initial Build"
    
    cd "$REDOX_DIR"
    
    print_info "Building Redox kernel (this will take a while)..."
    print_warning "This is a test build to verify everything works"
    
    if make kernel; then
        print_success "Kernel built successfully!"
    else
        print_error "Kernel build failed"
        print_info "Check the error messages above"
        exit 1
    fi
}

create_build_script() {
    print_header "Creating Build Scripts"
    
    # Create main build script
    cat > "$WORK_DIR/build-vantis.sh" << 'EOF'
#!/bin/bash
# VantisOS Build Script

set -e

REDOX_DIR="$HOME/vantis-build/redox-base"
cd "$REDOX_DIR"

echo "Building VantisOS..."

# Build with VantisOS config
make FILESYSTEM_CONFIG=config/x86_64/vantis.toml all

echo "Build complete!"
echo "ISO location: $REDOX_DIR/build/livedisk.iso"
EOF

    chmod +x "$WORK_DIR/build-vantis.sh"
    print_success "Build script created: $WORK_DIR/build-vantis.sh"
    
    # Create test script
    cat > "$WORK_DIR/test-vantis.sh" << 'EOF'
#!/bin/bash
# VantisOS Test Script

REDOX_DIR="$HOME/vantis-build/redox-base"
ISO="$REDOX_DIR/build/livedisk.iso"

if [ ! -f "$ISO" ]; then
    echo "ISO not found. Build first with: ./build-vantis.sh"
    exit 1
fi

echo "Testing VantisOS in QEMU..."
qemu-system-x86_64 \
    -cdrom "$ISO" \
    -m 4G \
    -enable-kvm \
    -cpu host \
    -serial stdio
EOF

    chmod +x "$WORK_DIR/test-vantis.sh"
    print_success "Test script created: $WORK_DIR/test-vantis.sh"
}

print_next_steps() {
    print_header "Setup Complete!"
    
    echo ""
    print_success "VantisOS build environment is ready!"
    echo ""
    print_info "Next steps:"
    echo ""
    echo "1. Build VantisOS:"
    echo "   cd $WORK_DIR"
    echo "   ./build-vantis.sh"
    echo ""
    echo "2. Test in QEMU:"
    echo "   ./test-vantis.sh"
    echo ""
    echo "3. Build ISO only:"
    echo "   cd $REDOX_DIR"
    echo "   make FILESYSTEM_CONFIG=config/x86_64/vantis.toml iso"
    echo ""
    print_info "Documentation:"
    echo "   - Full build plan: $VANTIS_DIR/FULL_BUILD_PLAN.md"
    echo "   - Quick guide: $VANTIS_DIR/QUICK_BUILD_ISO_GUIDE.md"
    echo "   - Redox docs: https://doc.redox-os.org/book/"
    echo ""
    print_warning "Note: First build will take 30-60 minutes"
    echo ""
}

# Main execution
main() {
    clear
    print_header "VantisOS Full Build Setup"
    echo ""
    print_info "This script will:"
    echo "  1. Check dependencies"
    echo "  2. Clone Redox OS"
    echo "  3. Integrate VantisOS components"
    echo "  4. Create build scripts"
    echo "  5. Test initial build"
    echo ""
    print_warning "This will take 30-60 minutes and use ~10GB disk space"
    echo ""
    read -p "Continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Setup cancelled"
        exit 0
    fi
    
    echo ""
    
    # Run setup steps
    check_dependencies
    check_disk_space
    setup_work_directory
    clone_redox
    install_redox_dependencies
    integrate_vantis_components
    create_vantis_config
    create_build_script
    
    # Optional: test build
    echo ""
    print_info "Setup complete! Would you like to test the build now?"
    read -p "Run test build? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        test_build
    else
        print_info "Skipping test build"
    fi
    
    echo ""
    print_next_steps
}

# Run main function
main "$@"