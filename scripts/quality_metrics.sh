#!/usr/bin/env bash
#
# Code Quality Metrics Collector for VantisOS
# Collects and reports on various code quality metrics
#
# Usage: ./scripts/quality_metrics.sh [OPTIONS]
#
# Options:
#   --format FORMAT    Output format: text, json, markdown (default: markdown)
#   --output FILE      Save output to file
#   --strict           Exit with error if quality thresholds are not met
#   --help             Show this help message

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
FORMAT="markdown"
OUTPUT_FILE=""
STRICT_MODE=false
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
METRICS_FILE="$PROJECT_ROOT/metrics/report.md"

# Quality thresholds
THRESHOLDS=(
    "clippy_warnings:0"
    "cargo_check_errors:0"
    "code_coverage:70"
    "documentation_coverage:80"
    "max_function_length:100"
    "max_cyclomatic_complexity:20"
)

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --format)
            FORMAT="$2"
            shift 2
            ;;
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --strict)
            STRICT_MODE=true
            shift
            ;;
        --help)
            echo "Code Quality Metrics Collector for VantisOS"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --format FORMAT    Output format: text, json, markdown (default: markdown)"
            echo "  --output FILE      Save output to file"
            echo "  --strict           Exit with error if quality thresholds are not met"
            echo "  --help             Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Logging functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Metrics storage
declare -A METRICS

# Collect metrics
collect_clippy_warnings() {
    log_info "Collecting clippy warnings..."
    local count=0
    if command -v cargo &> /dev/null; then
        count=$(cargo clippy --all-targets --all-features --message-format=short 2>&1 | grep -c "warning:" || echo "0")
    fi
    METRICS[clippy_warnings]=$count
    log_info "Clippy warnings: $count"
}

collect_cargo_check_errors() {
    log_info "Checking cargo build..."
    local count=0
    if command -v cargo &> /dev/null; then
        if cargo check --all-targets 2>&1 | grep -q "error\["; then
            count=$(cargo check --all-targets 2>&1 | grep -c "error\[" || echo "0")
        fi
    fi
    METRICS[cargo_check_errors]=$count
    log_info "Cargo check errors: $count"
}

collect_code_coverage() {
    log_info "Collecting code coverage..."
    local coverage=0
    if [ -f "$PROJECT_ROOT/target/tarpaulin/lcov.info" ]; then
        coverage=$(grep -E "^LF:" "$PROJECT_ROOT/target/tarpaulin/lcov.info" | awk -F: '{sum+=$2} END {if(NR>0) print int(sum/NR); else print 0}')
    fi
    METRICS[code_coverage]=$coverage
    log_info "Code coverage: ${coverage}%"
}

collect_documentation_coverage() {
    log_info "Collecting documentation coverage..."
    local coverage=0
    if command -v cargo &> /dev/null; then
        local total=0
        local documented=0
        # Count documented vs total public items
        total=$(cargo doc --no-deps 2>&1 | grep -oE "[0-9]+ items" | head -1 | grep -oE "[0-9]+" || echo "0")
        if [ "$total" -gt 0 ]; then
            documented=$(cargo doc --no-deps 2>&1 | grep -oE "[0-9]+ documented" | head -1 | grep -oE "[0-9]+" || echo "0")
            coverage=$((documented * 100 / total))
        fi
    fi
    METRICS[documentation_coverage]=$coverage
    log_info "Documentation coverage: ${coverage}%"
}

collect_code_statistics() {
    log_info "Collecting code statistics..."
    
    local total_lines=0
    local rust_lines=0
    local shell_lines=0
    local markdown_lines=0
    
    if [ -d "$PROJECT_ROOT" ]; then
        total_lines=$(find "$PROJECT_ROOT" -type f -name "*.rs" -o -name "*.sh" -o -name "*.md" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
        rust_lines=$(find "$PROJECT_ROOT" -type f -name "*.rs" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
        shell_lines=$(find "$PROJECT_ROOT" -type f -name "*.sh" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
        markdown_lines=$(find "$PROJECT_ROOT" -type f -name "*.md" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    fi
    
    METRICS[total_lines]=$total_lines
    METRICS[rust_lines]=$rust_lines
    METRICS[shell_lines]=$shell_lines
    METRICS[markdown_lines]=$markdown_lines
    
    log_info "Total lines: $total_lines (Rust: $rust_lines, Shell: $shell_lines, MD: $markdown_lines)"
}

collect_complexity_metrics() {
    log_info "Collecting complexity metrics..."
    
    local max_function_length=0
    local avg_function_length=0
    local complex_functions=0
    
    if command -v cargo &> /dev/null; then
        # Try to get complexity metrics
        if command -v cargo-complexity &> /dev/null; then
            local complexity_output=$(cargo complexity --threshold 20 2>&1 || echo "")
            if echo "$complexity_output" | grep -q "complexity"; then
                # Parse complexity output
                max_function_length=$(echo "$complexity_output" | grep -oE "max function length: [0-9]+" | grep -oE "[0-9]+" || echo "0")
                avg_function_length=$(echo "$complexity_output" | grep -oE "avg function length: [0-9]+" | grep -oE "[0-9]+" || echo "0")
            fi
        fi
    fi
    
    # Count functions longer than threshold
    if [ -d "$PROJECT_ROOT" ]; then
        complex_functions=$(find "$PROJECT_ROOT" -name "*.rs" -exec awk '
            /^[[:space:]]*pub[[:space:]]+fn[[:space:]]/ { 
                func_start = NR 
                brace_count = 0
                in_func = 1
            }
            in_func && /\{/ { brace_count++ }
            in_func && /\}/ { 
                brace_count--
                if (brace_count == 0) {
                    length = NR - func_start
                    if (length > 100) complex++
                    in_func = 0
                }
            }
            END { print complex }
        ' {} + 2>/dev/null || echo "0")
    fi
    
    METRICS[max_function_length]=$max_function_length
    METRICS[avg_function_length]=$avg_function_length
    METRICS[complex_functions]=$complex_functions
    
    log_info "Max function length: $max_function_length, Avg: $avg_function_length, Complex functions: $complex_functions"
}

collect_git_metrics() {
    log_info "Collecting git metrics..."
    
    local total_commits=0
    local contributors=0
    local branches=0
    
    if [ -d "$PROJECT_ROOT/.git" ]; then
        total_commits=$(git -C "$PROJECT_ROOT" rev-list --count HEAD 2>/dev/null || echo "0")
        contributors=$(git -C "$PROJECT_ROOT" shortlog -sn 2>/dev/null | wc -l || echo "0")
        branches=$(git -C "$PROJECT_ROOT" branch -r 2>/dev/null | wc -l || echo "0")
    fi
    
    METRICS[total_commits]=$total_commits
    METRICS[contributors]=$contributors
    METRICS[branches]=$branches
    
    log_info "Commits: $total_commits, Contributors: $contributors, Branches: $branches"
}

check_thresholds() {
    log_info "Checking quality thresholds..."
    
    local failed=0
    
    for threshold in "${THRESHOLDS[@]}"; do
        IFS=':' read -r metric_name expected_value <<< "$threshold"
        actual_value="${METRICS[$metric_name]:-0}"
        
        # Determine if metric should be less than or greater than threshold
        case "$metric_name" in
            *_warnings|*_errors|max_*|complex_*)
                # Should be less than or equal
                if [ "$actual_value" -gt "$expected_value" ]; then
                    log_error "$metric_name: $actual_value > $expected_value (FAIL)"
                    ((failed++))
                else
                    log_success "$metric_name: $actual_value <= $expected_value (PASS)"
                fi
                ;;
            *_coverage)
                # Should be greater than or equal
                if [ "$actual_value" -lt "$expected_value" ]; then
                    log_error "$metric_name: $actual_value% < $expected_value% (FAIL)"
                    ((failed++))
                else
                    log_success "$metric_name: $actual_value% >= $expected_value% (PASS)"
                fi
                ;;
            *)
                log_warning "Unknown threshold: $metric_name"
                ;;
        esac
    done
    
    if [ "$failed" -gt 0 ]; then
        log_error "$failed threshold(s) failed"
        if [ "$STRICT_MODE" = true ]; then
            exit 1
        fi
    else
        log_success "All thresholds passed"
    fi
}

generate_report_text() {
    local report=""
    report+="# Code Quality Metrics Report\n"
    report+="Generated: $(date)\n"
    report+="\n"
    report+="## Metrics Summary\n"
    report+="\n"
    report+="| Metric | Value | Threshold | Status |\n"
    report+="|--------|-------|-----------|--------|\n"
    report+="| Clippy Warnings | ${METRICS[clippy_warnings]:-0} | 0 | $(check_status ${METRICS[clippy_warnings]:-0} 0 le) |\n"
    report+="| Cargo Check Errors | ${METRICS[cargo_check_errors]:-0} | 0 | $(check_status ${METRICS[cargo_check_errors]:-0} 0 le) |\n"
    report+="| Code Coverage | ${METRICS[code_coverage]:-0}% | 70% | $(check_status ${METRICS[code_coverage]:-0} 70 ge) |\n"
    report+="| Documentation Coverage | ${METRICS[documentation_coverage]:-0}% | 80% | $(check_status ${METRICS[documentation_coverage]:-0} 80 ge) |\n"
    report+="\n"
    report+="## Code Statistics\n"
    report+="\n"
    report+="| Statistic | Value |\n"
    report+="|-----------|-------|\n"
    report+="| Total Lines of Code | ${METRICS[total_lines]:-0} |\n"
    report+="| Rust Lines | ${METRICS[rust_lines]:-0} |\n"
    report+="| Shell Lines | ${METRICS[shell_lines]:-0} |\n"
    report+="| Documentation Lines | ${METRICS[markdown_lines]:-0} |\n"
    report+="| Max Function Length | ${METRICS[max_function_length]:-0} |\n"
    report+="| Avg Function Length | ${METRICS[avg_function_length]:-0} |\n"
    report+="| Complex Functions (>100 lines) | ${METRICS[complex_functions]:-0} |\n"
    report+="\n"
    report+="## Git Statistics\n"
    report+="\n"
    report+="| Statistic | Value |\n"
    report+="|-----------|-------|\n"
    report+="| Total Commits | ${METRICS[total_commits]:-0} |\n"
    report+="| Contributors | ${METRICS[contributors]:-0} |\n"
    report+="| Branches | ${METRICS[branches]:-0} |\n"
    
    echo "$report"
}

check_status() {
    local actual=$1
    local expected=$2
    local op=$3
    
    case "$op" in
        le) # less than or equal
            if [ "$actual" -le "$expected" ]; then
                echo "✅ PASS"
            else
                echo "❌ FAIL"
            fi
            ;;
        ge) # greater than or equal
            if [ "$actual" -ge "$expected" ]; then
                echo "✅ PASS"
            else
                echo "❌ FAIL"
            fi
            ;;
    esac
}

save_report() {
    local report="$1"
    mkdir -p "$(dirname "$METRICS_FILE")"
    echo "$report" > "$METRICS_FILE"
    log_success "Report saved to $METRICS_FILE"
}

main() {
    echo "=========================================="
    echo "  Code Quality Metrics Collector"
    echo "=========================================="
    echo ""
    
    # Collect all metrics
    collect_clippy_warnings
    collect_cargo_check_errors
    collect_code_coverage
    collect_documentation_coverage
    collect_code_statistics
    collect_complexity_metrics
    collect_git_metrics
    
    echo ""
    
    # Check thresholds
    check_thresholds
    
    echo ""
    
    # Generate report
    local report=$(generate_report_text)
    
    if [ -n "$OUTPUT_FILE" ]; then
        echo "$report" > "$OUTPUT_FILE"
        log_success "Report saved to $OUTPUT_FILE"
    else
        echo ""
        echo "=========================================="
        echo "  Metrics Report"
        echo "=========================================="
        echo ""
        echo "$report"
    fi
    
    # Always save to metrics directory
    save_report "$report"
    
    echo ""
    log_success "Metrics collection completed"
}

main "$@"