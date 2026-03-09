#!/bin/bash
#
# VantisOS v1.4.0 Deployment Script
# 
# Usage: ./deploy.sh [environment] [version]
#   environment: staging | production (default: staging)
#   version: version tag (default: latest)
#

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
APP_NAME="vantis"
DEFAULT_ENVIRONMENT="staging"
DEFAULT_VERSION="latest"

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
    echo "  VantisOS v1.4.0 Deployment Script"
    echo "========================================"
    echo ""
}

# Parse arguments
parse_args() {
    ENVIRONMENT="${1:-$DEFAULT_ENVIRONMENT}"
    VERSION="${2:-$DEFAULT_VERSION}"
    
    log_info "Environment: $ENVIRONMENT"
    log_info "Version: $VERSION"
}

# Validate prerequisites
validate_prerequisites() {
    log_info "Validating prerequisites..."
    
    # Check required commands
    local required_commands=("docker" "kubectl" "helm")
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
    
    # Check Docker is running
    if ! docker info &> /dev/null; then
        log_error "Docker is not running"
        exit 1
    fi
    
    # Check Kubernetes context
    local current_context
    current_context=$(kubectl config current-context)
    log_info "Current Kubernetes context: $current_context"
    
    log_success "Prerequisites validated"
}

# Build application
build_application() {
    log_info "Building application..."
    
    cd "$PROJECT_ROOT"
    
    # Build release binary
    cargo build --release
    
    # Run tests
    log_info "Running tests..."
    cargo test --release
    
    log_success "Application built successfully"
}

# Build Docker image
build_docker_image() {
    log_info "Building Docker image..."
    
    cd "$PROJECT_ROOT"
    
    local image_tag="${APP_NAME}:${VERSION}"
    
    if [ "$ENVIRONMENT" == "production" ]; then
        image_tag="${APP_NAME}:${VERSION}-prod"
    fi
    
    docker build \
        -t "$image_tag" \
        -t "${APP_NAME}:latest" \
        --build-arg VERSION="$VERSION" \
        --build-arg BUILD_DATE="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
        .
    
    log_success "Docker image built: $image_tag"
    
    # Export for later use
    DOCKER_IMAGE="$image_tag"
}

# Push Docker image
push_docker_image() {
    log_info "Pushing Docker image..."
    
    local registry="${DOCKER_REGISTRY:-ghcr.io}"
    local repo="${registry}/vantiscorp/${APP_NAME}"
    
    # Tag for registry
    docker tag "$DOCKER_IMAGE" "${repo}:${VERSION}"
    docker tag "$DOCKER_IMAGE" "${repo}:latest"
    
    # Push to registry
    docker push "${repo}:${VERSION}"
    docker push "${repo}:latest"
    
    log_success "Docker image pushed: ${repo}:${VERSION}"
    
    # Export for later use
    DEPLOYMENT_IMAGE="${repo}:${VERSION}"
}

# Deploy to Kubernetes
deploy_kubernetes() {
    log_info "Deploying to Kubernetes..."
    
    local namespace="${APP_NAME}-${ENVIRONMENT}"
    local values_file="${PROJECT_ROOT}/helm/values-${ENVIRONMENT}.yaml"
    
    # Create namespace if it doesn't exist
    kubectl create namespace "$namespace" --dry-run=client -o yaml | kubectl apply -f -
    
    # Deploy using Helm
    helm upgrade --install "${APP_NAME}" "${PROJECT_ROOT}/helm/${APP_NAME}" \
        --namespace "$namespace" \
        --set image.repository="${DEPLOYMENT_IMAGE%:*}" \
        --set image.tag="${DEPLOYMENT_IMAGE#*:}" \
        --set environment="$ENVIRONMENT" \
        -f "$values_file" \
        --wait \
        --timeout 10m
    
    log_success "Deployed to Kubernetes namespace: $namespace"
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
    
    # Wait for deployment to be ready
    kubectl rollout status deployment/"${APP_NAME}" -n "$namespace" --timeout=5m
    
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

# Run smoke tests
run_smoke_tests() {
    log_info "Running smoke tests..."
    
    local namespace="${APP_NAME}-${ENVIRONMENT}"
    
    # Run smoke tests
    kubectl exec -n "$namespace" deployment/"${APP_NAME}" -- \
        /app/vantis smoke-test
    
    log_success "Smoke tests passed"
}

# Record deployment
record_deployment() {
    log_info "Recording deployment..."
    
    local deploy_id
    deploy_id=$(uuidgen)
    
    # Create deployment record
    cat > "${PROJECT_ROOT}/deployments/${deploy_id}.json" << EOF
{
    "id": "$deploy_id",
    "version": "$VERSION",
    "environment": "$ENVIRONMENT",
    "image": "$DEPLOYMENT_IMAGE",
    "timestamp": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')",
    "deployer": "${USER:-unknown}",
    "status": "success"
}
EOF
    
    log_success "Deployment recorded: $deploy_id"
}

# Send notification
send_notification() {
    log_info "Sending notification..."
    
    local message="VantisOS ${VERSION} deployed to ${ENVIRONMENT}"
    
    # Slack notification (if webhook is configured)
    if [ -n "${SLACK_WEBHOOK:-}" ]; then
        curl -X POST -H 'Content-type: application/json' \
            --data "{&quot;text&quot;:&quot;$message&quot;}" \
            "$SLACK_WEBHOOK"
    fi
    
    # Email notification (if configured)
    if [ -n "${NOTIFICATION_EMAIL:-}" ]; then
        echo "$message" | mail -s "VantisOS Deployment" "$NOTIFICATION_EMAIL"
    fi
    
    log_success "Notification sent"
}

# Main deployment function
main() {
    print_banner
    parse_args "$@"
    
    log_info "Starting deployment..."
    
    # Deployment steps
    validate_prerequisites
    build_application
    build_docker_image
    
    if [ "$ENVIRONMENT" == "production" ]; then
        push_docker_image
    fi
    
    deploy_kubernetes
    run_health_checks
    run_smoke_tests
    record_deployment
    send_notification
    
    log_success "Deployment completed successfully!"
    echo ""
    echo "Deployment Summary:"
    echo "  - Environment: $ENVIRONMENT"
    echo "  - Version: $VERSION"
    echo "  - Image: $DEPLOYMENT_IMAGE"
    echo ""
}

# Run main function
main "$@"