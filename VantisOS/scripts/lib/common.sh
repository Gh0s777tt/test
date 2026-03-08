#!/bin/bash
# Common library for VantisOS scripts
# Provides standardized logging, error handling, and utility functions

# Colors for output
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export BLUE='\033[0;34m'
export CYAN='\033[0;36m'
export NC='\033[0m' # No Color
export BOLD='\033[1m'

# Logging configuration
export LOG_LEVEL="${LOG_LEVEL:-INFO}"
export LOG_FORMAT="${LOG_FORMAT:-text}"  # text or json

# Log levels
declare -A LOG_LEVELS=(
    ["DEBUG"]=0
    ["INFO"]=1
    ["WARN"]=2
    ["ERROR"]=3
    ["FATAL"]=4
)

# Get current timestamp
get_timestamp() {
    date '+%Y-%m-%d %H:%M:%S'
}

# Get script name
get_script_name() {
    basename "$0"
}

# Logging function
# Usage: log LEVEL "Message"
log() {
    local level="$1"
    local message="$2"
    local script_name
    script_name=$(get_script_name)
    
    # Check if this log level should be shown
    if [[ ${LOG_LEVELS[$level]} -ge ${LOG_LEVELS[$LOG_LEVEL]} ]]; then
        local timestamp
        timestamp=$(get_timestamp)
        
        if [[ "$LOG_FORMAT" == "json" ]]; then
            echo "{&quot;timestamp&quot;: &quot;$timestamp&quot;, &quot;level&quot;: &quot;$level&quot;, &quot;script&quot;: &quot;$script_name&quot;, &quot;message&quot;: &quot;$message&quot;}"
        else
            local color=""
            case $level in
                DEBUG) color="$CYAN" ;;
                INFO) color="$GREEN" ;;
                WARN) color="$YELLOW" ;;
                ERROR) color="$RED" ;;
                FATAL) color="$RED$BOLD" ;;
            esac
            echo -e "${color}[$timestamp] [$level] [$script_name]${NC} $message"
        fi
    fi
}

# Convenience logging functions
log_debug() { log "DEBUG" "$1"; }
log_info() { log "INFO" "$1"; }
log_warn() { log "WARN" "$1"; }
log_error() { log "ERROR" "$1"; }
log_fatal() { log "FATAL" "$1"; }

# Print a section header
# Usage: print_header "Section Title"
print_header() {
    echo ""
    echo -e "${BOLD}${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}${BLUE}  $1${NC}"
    echo -e "${BOLD}${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
}

# Print a sub-header
# Usage: print_subheader "Subsection Title"
print_subheader() {
    echo ""
    echo -e "${BOLD}${CYAN}── $1 ──${NC}"
    echo ""
}

# Print success message
# Usage: print_success "Operation completed successfully"
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Print error message
# Usage: print_error "Operation failed"
print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Print warning message
# Usage: print_warning "Warning message"
print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Print info message
# Usage: print_info "Information message"
print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Check if a command exists
# Usage: check_command "git"
check_command() {
    if ! command -v "$1" &> /dev/null; then
        log_error "Required command '$1' not found"
        return 1
    fi
    log_debug "Command '$1' found"
    return 0
}

# Check multiple commands at once
# Usage: check_commands "git" "docker" "make"
check_commands() {
    local missing=()
    for cmd in "$@"; do
        if ! check_command "$cmd"; then
            missing+=("$cmd")
        fi
    done
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        log_error "Missing required commands: ${missing[*]}"
        return 1
    fi
    return 0
}

# Check if a file exists
# Usage: check_file "/path/to/file"
check_file() {
    if [[ ! -f "$1" ]]; then
        log_error "File not found: $1"
        return 1
    fi
    log_debug "File found: $1"
    return 0
}

# Check if a directory exists
# Usage: check_dir "/path/to/dir"
check_dir() {
    if [[ ! -d "$1" ]]; then
        log_error "Directory not found: $1"
        return 1
    fi
    log_debug "Directory found: $1"
    return 0
}

# Create directory if it doesn't exist
# Usage: ensure_dir "/path/to/dir"
ensure_dir() {
    if [[ ! -d "$1" ]]; then
        log_info "Creating directory: $1"
        mkdir -p "$1"
    fi
}

# Check disk space
# Usage: check_disk_space "/path" min_gb
check_disk_space() {
    local path="$1"
    local min_gb="$2"
    
    if [[ ! -d "$path" ]]; then
        log_error "Path does not exist: $path"
        return 1
    fi
    
    local available_kb
    available_kb=$(df -k "$path" | awk 'NR==2 {print $4}')
    local available_gb=$((available_kb / 1024 / 1024))
    
    if [[ $available_gb -lt $min_gb ]]; then
        log_error "Insufficient disk space. Required: ${min_gb}GB, Available: ${available_gb}GB"
        return 1
    fi
    
    log_debug "Disk space check passed. Available: ${available_gb}GB"
    return 0
}

# Validate input arguments
# Usage: validate_args min_args "$@"
validate_args() {
    local min_args="$1"
    shift
    local actual_args=$#
    
    if [[ $actual_args -lt $min_args ]]; then
        log_error "Insufficient arguments. Required: $min_args, Provided: $actual_args"
        return 1
    fi
    return 0
}

# Run a command with error handling
# Usage: run_cmd "Description" command args...
run_cmd() {
    local description="$1"
    shift
    local cmd=("$@")
    
    log_info "Running: $description"
    log_debug "Command: ${cmd[*]}"
    
    if "${cmd[@]}"; then
        print_success "$description completed"
        return 0
    else
        print_error "$description failed"
        return 1
    fi
}

# Run a command silently
# Usage: run_silent "Description" command args...
run_silent() {
    local description="$1"
    shift
    local cmd=("$@")
    
    log_debug "Running silently: $description"
    log_debug "Command: ${cmd[*]}"
    
    if "${cmd[@]}" &> /dev/null; then
        log_debug "$description completed"
        return 0
    else
        log_error "$description failed"
        return 1
    fi
}

# Confirm action with user
# Usage: confirm "Are you sure?" && do_something
confirm() {
    local message="${1:-Are you sure?}"
    echo -en "${YELLOW}$message [y/N] ${NC}"
    read -r response
    case "$response" in
        [yY][eE][sS]|[yY])
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

# Show usage from script header
# Usage: show_usage
show_usage() {
    grep '^# Usage:' "$0" | head -1 | sed 's/^# Usage: //'
}

# Show help from script header
# Usage: show_help
show_help() {
    echo ""
    grep '^#' "$0" | head -20 | sed 's/^# //'
    echo ""
}

# Cleanup function to be called on script exit
# Usage: trap cleanup EXIT
cleanup() {
    local exit_code=$?
    if [[ $exit_code -ne 0 ]]; then
        log_error "Script exited with code: $exit_code"
    fi
}

# Error handler
# Usage: trap error_handler ERR
error_handler() {
    local exit_code=$?
    local line_no=$1
    log_error "Error on line $line_no (exit code: $exit_code)"
    exit $exit_code
}

# Setup common traps
# Usage: setup_traps
setup_traps() {
    trap cleanup EXIT
    trap 'error_handler $LINENO' ERR
}

# Get operating system
# Usage: os=$(get_os)
get_os() {
    case "$(uname -s)" in
        Linux*) echo "linux" ;;
        Darwin*) echo "macos" ;;
        CYGWIN*|MINGW*|MSYS*) echo "windows" ;;
        *) echo "unknown" ;;
    esac
}

# Get architecture
# Usage: arch=$(get_arch)
get_arch() {
    case "$(uname -m)" in
        x86_64|amd64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        armv7l|armhf) echo "armv7" ;;
        riscv64) echo "riscv64" ;;
        *) echo "unknown" ;;
    esac
}

# Check if running as root
# Usage: if is_root; then ...
is_root() {
    [[ $EUID -eq 0 ]]
}

# Ensure running as root
# Usage: ensure_root
ensure_root() {
    if ! is_root; then
        log_error "This script must be run as root"
        exit 1
    fi
}

# Progress indicator
# Usage: show_progress "Doing something"
#        # do work
#        hide_progress
show_progress() {
    local message="${1:-Processing}"
    echo -n "$message..."
}

hide_progress() {
    echo " done"
}

# Parse version string
# Usage: parse_version "1.2.3" major minor patch
parse_version() {
    local version="$1"
    local -n major_ref=$2
    local -n minor_ref=$3
    local -n patch_ref=$4
    
    IFS='.' read -r major_ref minor_ref patch_ref <<< "$version"
}

# Compare versions
# Usage: compare_versions "1.2.3" "1.2.4"
# Returns: 0 if equal, 1 if first > second, 2 if first < second
compare_versions() {
    if [[ "$1" == "$2" ]]; then
        return 0
    fi
    
    local IFS=.
    local i ver1=($1) ver2=($2)
    
    for ((i=0; i<${#ver1[@]}; i++)); do
        if [[ -z ${ver2[i]} ]]; then
            ver2[i]=0
        fi
        
        if ((10#${ver1[i]} > 10#${ver2[i]})); then
            return 1
        fi
        if ((10#${ver1[i]} < 10#${ver2[i]})); then
            return 2
        fi
    done
    return 0
}

# Download file with progress
# Usage: download_file "url" "output_file"
download_file() {
    local url="$1"
    local output="$2"
    
    log_info "Downloading: $url"
    
    if command -v wget &> /dev/null; then
        wget -q --show-progress -O "$output" "$url"
    elif command -v curl &> /dev/null; then
        curl -# -L -o "$output" "$url"
    else
        log_error "Neither wget nor curl found"
        return 1
    fi
}

# Calculate checksum
# Usage: calc_checksum "file" "algorithm"
calc_checksum() {
    local file="$1"
    local algo="${2:-sha256}"
    
    case $algo in
        md5) md5sum "$file" | cut -d' ' -f1 ;;
        sha1) sha1sum "$file" | cut -d' ' -f1 ;;
        sha256) sha256sum "$file" | cut -d' ' -f1 ;;
        sha512) sha512sum "$file" | cut -d' ' -f1 ;;
        *) log_error "Unknown checksum algorithm: $algo"; return 1 ;;
    esac
}

# Backup file
# Usage: backup_file "file"
backup_file() {
    local file="$1"
    if [[ -f "$file" ]]; then
        local backup="${file}.bak.$(date +%Y%m%d_%H%M%S)"
        cp "$file" "$backup"
        log_info "Backup created: $backup"
        echo "$backup"
    fi
}

# Wait for process
# Usage: wait_for_pid 1234
wait_for_pid() {
    local pid="$1"
    while kill -0 "$pid" 2>/dev/null; do
        sleep 1
    done
}

# Export functions for use in subshells
export -f log log_debug log_info log_warn log_error log_fatal
export -f print_header print_subheader print_success print_error print_warning print_info
export -f check_command check_commands check_file check_dir ensure_dir
export -f check_disk_space validate_args run_cmd run_silent
export -f confirm show_usage show_help cleanup error_handler setup_traps
export -f get_os get_arch is_root ensure_root
export -f show_progress hide_progress parse_version compare_versions
export -f download_file calc_checksum backup_file wait_for_pid