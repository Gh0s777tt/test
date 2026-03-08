#!/bin/bash
#
# VantisOS v1.4.0 Rollback Script
# 
# Usage: ./rollback.sh [environment] [revision]
#   environment: staging | production (default: staging)
#   revision: revision number or "previous" (default: previous)
#

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
APP_NAME="vantis"
DEFAULT_ENVIRONMENT="staging"
DEFAULT_REVISION="previous"
MAX_REVISIONS=10

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Print banner
print_banner() {
    echo "========================================"
    echo "  VantisOS v1.4.0 Rollback Script"
    echo "========================================"
    echo ""
}

# Parse arguments
parse_args() {
    ENVIRONMENT="${1:-$DEFAULT_ENVIRONMENT}"
    REVISION="${2:-$DEFAULT_REVISION}"
    
    log_info "Environment: $ENVIRONMENT"
    log_info "Target Revision: $REVISION"
}

# Validate prerequisites
validate_prerequisites() {
    log_info "Validating prerequisites..."
    
    # Check required commands
    local required_commands=("kubectl" "helm")
    local missing_commands=()
    
    for cmd in "${required_commands[@]}"; do
        if ! command -v "$cmd" &> /dev/null; then
            missing_commands+=("$cmd")
        fi
    done
    
    if [ ${#missing_commands[@]} -gt 0 ]; then
        log_error "Missing required commands: ${missing_commands[*]}"
        exit 1
    fi
    
    # Check Kubernetes context
    local current_context
    current_context=$(kubectl config current-context)
    log_info "Current Kubernetes context: $current_context"
    
    log_success "Prerequisites validated"
}

# Get deployment history
get_deployment_history() {
    log_info "Getting deployment history..."
    
    local namespace="${APP_NAME}-${ENVIRONMENT}"
    
    # Get Helm history
    echo ""
    echo "Recent deployments:"
    echo "-------------------"
    helm history "${APP_NAME}" -n "$namespace" --max "$MAX_REVISIONS"
    echo ""
}

# Get current version
get_current_version() {
    local namespace="${APP_NAME}-${ENVIRONMENT}"
    
    CURRENT_VERSION=$(kubectl get deployment "${APP_NAME}" -n "$namespace" \
        -o jsonpath='{.spec.template.spec.containers[0].image}' | cut -d: -f2)
    
    log_info "Current version: $CURRENT_VERSION"
}

# Confirm rollback
confirm_rollback() {
    log_warning "You are about to rollback to revision: $REVISION"
    log_warning "This will revert to a previous version of the application."
    echo ""
    
    read -p "Are you sure you want to proceed? (y/N): " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Rollback cancelled by user"
        exit 0
    fi
}

# Perform rollback
perform_rollback() {
    log_info "Performing rollback..."
    
    local namespace="${APP_NAME}-${ENVIRONMENT}"
    local target_revision="$REVISION"
    
    # If "previous", get the previous revision number
    if [ "$target_revision" == "previous" ]; then
        local current_revision
        current_revision=$(helm history "${APP_NAME}" -n "$namespace" -o json | \
            jq -r '.[-1].revision')
        target_revision=$((current_revision - 1))
        log_info "Rolling back to revision: $target_revision"
    fi
    
    # Perform Helm rollback
    helm rollback "${APP_NAME}" "$target_revision" \
        -n "$namespace" \
        --wait \
        --timeout 5m
    
    log_success "Rollback completed"
}

# Verify rollback
verify_rollback() {
    log_info "Verifying rollback..."
    
    local namespace="${APP_NAME}-${ENVIRONMENT}"
    
    # Wait for deployment to be ready
    kubectl rollout status deployment/"${APP_NAME}" -n "$namespace" --timeout=5m
    
    # Check pods are running
    local ready_pods
    ready_pods=$(kubectl get pods -n "$namespace" \
        -l "app.kubernetes.io/name=${APP_NAME}" \
        -o jsonpath='{.items[*].status.conditions[?(@.type=="Ready")].status}' | \
        tr ' ' '\n' | grep -c "True" || true)
    
    if [ "$ready_pods" -gt 0 ]; then
        log_success "Rollback verified: $ready_pods pods ready"
    else
        log_error "Rollback verification failed: no pods ready"
        return 1
    fi
}

# Run health checks
run_health_checks() {
    log_info "Running health checks..."
    
    local namespace="${APP_NAME}-${ENVIRONMENT}"
    local service_url
    
    # Get service URL
    if [ "$ENVIRONMENT" == "production" ]; then
        service_url="https://vantis.vantis.ai"
    else
        service_url="https://vantis-staging.vantis.ai"
    fi
    
    # Health check
    local max_retries=30
    local retry_count=0
    local health_endpoint="${service_url}/health"
    
    while [ $retry_count -lt $max_retries ]; do
        if curl -sf "$health_endpoint" > /dev/null; then
            log_success "Health check passed"
            return 0
        fi
        
        retry_count=$((retry_count + 1))
        log_warning "Health check attempt $retry_count/$max_retries failed, retrying..."
        sleep 10
    done
    
    log_error "Health check failed after $max_retries attempts"
    return 1
}

# Get rollback version info
get_rollback_info() {
    local namespace="${APP_NAME}-${ENVIRONMENT}"
    
    NEW_VERSION=$(kubectl get deployment "${APP_NAME}" -n "$namespace" \
        -o jsonpath='{.spec.template.spec.containers[0].image}' | cut -d: -f2)
    
    log_info "Rolled back to version: $NEW_VERSION"
}

# Record rollback
record_rollback() {
    log_info "Recording rollback..."
    
    local rollback_id
    rollback_id=$(uuidgen)
    
    mkdir -p "${PROJECT_ROOT}/rollbacks"
    
    # Create rollback record
    cat > "${PROJECT_ROOT}/rollbacks/${rollback_id}.json" << EOF
{
    "id": "$rollback_id",
    "environment": "$ENVIRONMENT",
    "previous_version": "$CURRENT_VERSION",
    "new_version": "$NEW_VERSION",
    "revision": "$REVISION",
    "timestamp": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')",
    "initiator": "${USER:-unknown}",
    "status": "success"
}
EOF
    
    log_success "Rollback recorded: $rollback_id"
}

# Send notification
send_notification() {
    log_info "Sending notification..."
    
    local message="⚠️ VantisOS rolled back from ${CURRENT_VERSION} to ${NEW_VERSION} in ${ENVIRONMENT}"
    
    # Slack notification (if webhook is configured)
    if [ -n "${SLACK_WEBHOOK:-}" ]; then
        curl -X POST -H 'Content-type: application/json' \
            --data "{&quot;text&quot;:&quot;$message&quot;}" \
            "$SLACK_WEBHOOK"
    fi
    
    # Email notification (if configured)
    if [ -n "${NOTIFICATION_EMAIL:-}" ]; then
        echo "$message" | mail -s "VantisOS Rollback" "$NOTIFICATION_EMAIL"
    fi
    
    log_success "Notification sent"
}

# Print rollback summary
print_summary() {
    echo ""
    echo "========================================"
    echo "  Rollback Summary"
    echo "========================================"
    echo ""
    echo "  Environment:    $ENVIRONMENT"
    echo "  Previous:       $CURRENT_VERSION"
    echo "  Current:        $NEW_VERSION"
    echo "  Revision:       $REVISION"
    echo "  Timestamp:      $(date -u +'%Y-%m-%dT%H:%M:%SZ')"
    echo ""
    log_success "Rollback completed successfully!"
    echo ""
}

# Main rollback function
main() {
    print_banner
    parse_args "$@"
    
    log_warning "Starting rollback process..."
    
    # Rollback steps
    validate_prerequisites
    get_deployment_history
    get_current_version
    confirm_rollback
    perform_rollback
    verify_rollback
    run_health_checks
    get_rollback_info
    record_rollback
    send_notification
    print_summary
}

# Run main function
main "$@"