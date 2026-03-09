# VantisOS Deployment Guide

## Executive Summary

This document provides comprehensive guidance for deploying VantisOS to production environments, including prerequisites, deployment procedures, and troubleshooting.

**Version**: 1.0  
**Last Updated**: February 26, 2025  
**Deployment Type**: Kubernetes-based, GitOps  

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Deployment Process](#deployment-process)
4. [Configuration](#configuration)
5. [Verification](#verification)
6. [Rollback](#rollback)
7. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Tools

| Tool | Version | Purpose |
|------|---------|---------|
| kubectl | 1.28+ | Kubernetes CLI |
| helm | 3.12+ | Package manager |
| terraform | 1.5+ | Infrastructure as Code |
| docker | 24.0+ | Container runtime |
| git | 2.40+ | Version control |
| argocd | 2.8+ | GitOps tool |

### Required Access

- **Cloud Provider Account**: AWS, Azure, or GCP account with appropriate permissions
- **GitHub Repository**: Access to VantisOS repository
- **Container Registry**: Access to container registry (ECR, ACR, GCR)
- **DNS Provider**: Access to DNS management (Route53, Azure DNS, Cloud DNS)

### Required Knowledge

- Kubernetes fundamentals
- Docker and containerization
- CI/CD concepts
- Cloud provider basics
- Git and version control

---

## Environment Setup

### 1. Clone Repository

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS

# Checkout the correct branch
git checkout 0.4.1
```

### 2. Configure kubectl

```bash
# Configure kubectl for your cluster
aws eks update-kubeconfig --region us-east-1 --name vantisos-production

# Verify connection
kubectl cluster-info
kubectl get nodes
```

### 3. Install Helm

```bash
# Install Helm
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash

# Verify installation
helm version
```

### 4. Install ArgoCD

```bash
# Install ArgoCD
kubectl create namespace argocd
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Wait for ArgoCD to be ready
kubectl wait --for=condition=available --timeout=600s deployment/argocd-server -n argocd

# Access ArgoCD UI
kubectl port-forward svc/argocd-server -n argocd 8080:443
```

### 5. Install Monitoring Stack

```bash
# Add Prometheus Helm repository
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update

# Install Prometheus
helm install prometheus prometheus-community/kube-prometheus-stack -n monitoring --create-namespace

# Install Grafana
helm install grafana prometheus-community/grafana -n monitoring
```

---

## Deployment Process

### Overview

The deployment process follows these stages:

1. **Infrastructure Provisioning**: Provision cloud infrastructure using Terraform
2. **Application Build**: Build and push Docker images
3. **Configuration**: Configure application settings
4. **Deployment**: Deploy to Kubernetes using ArgoCD
5. **Verification**: Verify deployment success
6. **Monitoring**: Monitor application health

### Stage 1: Infrastructure Provisioning

```bash
# Navigate to Terraform directory
cd infrastructure/terraform

# Initialize Terraform
terraform init

# Plan infrastructure changes
terraform plan -out=tfplan

# Apply infrastructure changes
terraform apply tfplan

# Get outputs
terraform output
```

### Stage 2: Application Build

```bash
# Build Docker image
docker build -t vantisos:latest .

# Tag image for registry
docker tag vantisos:latest <registry>/vantisos:latest

# Push to registry
docker push <registry>/vantisos:latest
```

### Stage 3: Configuration

Create configuration files:

```yaml
# infrastructure/kubernetes/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: vantisos-config
  namespace: vantisos-production
data:
  DATABASE_URL: "postgresql://user:pass@db:5432/vantisos"
  REDIS_URL: "redis://redis:6379"
  LOG_LEVEL: "info"
  ENVIRONMENT: "production"
```

```yaml
# infrastructure/kubernetes/secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: vantisos-secrets
  namespace: vantisos-production
type: Opaque
data:
  DATABASE_PASSWORD: <base64-encoded-password>
  API_KEY: <base64-encoded-api-key>
```

### Stage 4: Deployment

#### Option 1: Using kubectl

```bash
# Apply configurations
kubectl apply -f infrastructure/kubernetes/configmap.yaml
kubectl apply -f infrastructure/kubernetes/secret.yaml

# Deploy application
kubectl apply -f infrastructure/kubernetes/deployment.yaml
kubectl apply -f infrastructure/kubernetes/service.yaml
kubectl apply -f infrastructure/kubernetes/ingress.yaml

# Wait for deployment to be ready
kubectl rollout status deployment/vantisos -n vantisos-production
```

#### Option 2: Using ArgoCD (GitOps)

```bash
# Create ArgoCD application
argocd app create vantisos \
  --repo https://github.com/vantisCorp/VantisOS.git \
  --path infrastructure/kubernetes \
  --dest-server https://kubernetes.default.svc \
  --dest-namespace vantisos-production \
  --revision 0.4.1

# Sync application
argocd app sync vantisos

# Wait for sync to complete
argocd app wait vantisos
```

### Stage 5: Verification

```bash
# Check deployment status
kubectl get deployment vantisos -n vantisos-production

# Check pods
kubectl get pods -n vantisos-production

# Check logs
kubectl logs -f deployment/vantisos -n vantisos-production

# Check services
kubectl get svc -n vantisos-production

# Check ingress
kubectl get ingress -n vantisos-production
```

### Stage 6: Monitoring

```bash
# Access Grafana
kubectl port-forward svc/grafana -n monitoring 3000:80

# Access Prometheus
kubectl port-forward svc/prometheus-kube-prometheus-prometheus -n monitoring 9090:9090

# Check metrics
kubectl top nodes
kubectl top pods -n vantisos-production
```

---

## Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | PostgreSQL connection string | - | Yes |
| `REDIS_URL` | Redis connection string | - | Yes |
| `LOG_LEVEL` | Logging level | `info` | No |
| `ENVIRONMENT` | Environment name | `production` | No |
| `API_KEY` | API authentication key | - | Yes |
| `SECRET_KEY` | Secret key for encryption | - | Yes |

### Resource Limits

```yaml
resources:
  requests:
    cpu: "500m"
    memory: "512Mi"
  limits:
    cpu: "2000m"
    memory: "2Gi"
```

### Autoscaling

```yaml
autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 10
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80
```

---

## Verification

### Health Checks

```bash
# Check application health
curl https://api.vantisos.com/health

# Expected response:
{
  "status": "healthy",
  "version": "0.4.1",
  "timestamp": "2025-02-26T00:00:00Z"
}
```

### Smoke Tests

```bash
# Run smoke tests
kubectl run smoke-test --image=curlimages/curl --rm -it --restart=Never -- \
  curl https://api.vantisos.com/health

# Test API endpoints
curl https://api.vantisos.com/api/v1/status
curl https://api.vantisos.com/api/v1/metrics
```

### Integration Tests

```bash
# Run integration tests
kubectl run integration-tests --image=vantisos:test --rm -it --restart=Never -- \
  pytest tests/integration/
```

---

## Rollback

### Manual Rollback

```bash
# View deployment history
kubectl rollout history deployment/vantisos -n vantisos-production

# Rollback to previous version
kubectl rollout undo deployment/vantisos -n vantisos-production

# Rollback to specific revision
kubectl rollout undo deployment/vantisos -n vantisos-production --to-revision=2

# Verify rollback
kubectl rollout status deployment/vantisos -n vantisos-production
```

### ArgoCD Rollback

```bash
# View application history
argocd app history vantisos

# Rollback to previous revision
argocd app sync vantisos --revision <previous-revision>

# Verify rollback
argocd app get vantisos
```

### Emergency Rollback

```bash
# Scale down deployment
kubectl scale deployment vantisos -n vantisos-production --replicas=0

# Restore from backup
kubectl apply -f backups/vantisos-deployment-backup.yaml

# Scale up deployment
kubectl scale deployment vantisos -n vantisos-production --replicas=3
```

---

## Troubleshooting

### Common Issues

#### 1. Pods Not Starting

**Symptoms**: Pods stuck in Pending or CrashLoopBackOff state

**Solutions**:
```bash
# Check pod status
kubectl describe pod <pod-name> -n vantisos-production

# Check pod logs
kubectl logs <pod-name> -n vantisos-production

# Check events
kubectl get events -n vantisos-production --sort-by='.lastTimestamp'
```

**Common Causes**:
- Insufficient resources
- Image pull errors
- Configuration errors
- Dependency failures

#### 2. Service Not Accessible

**Symptoms**: Cannot access service from external network

**Solutions**:
```bash
# Check service status
kubectl get svc vantisos -n vantisos-production

# Check service endpoints
kubectl get endpoints vantisos -n vantisos-production

# Check ingress
kubectl get ingress vantisos -n vantisos-production
kubectl describe ingress vantisos -n vantisos-production
```

**Common Causes**:
- Incorrect service type
- Missing ingress configuration
- DNS propagation issues
- Firewall rules

#### 3. Database Connection Issues

**Symptoms**: Application cannot connect to database

**Solutions**:
```bash
# Check database pod
kubectl get pods -n database

# Check database logs
kubectl logs -f deployment/postgres -n database

# Test database connection
kubectl run db-test --image=postgres:15 --rm -it --restart=Never -- \
  psql $DATABASE_URL
```

**Common Causes**:
- Incorrect connection string
- Network policies blocking traffic
- Database not ready
- Authentication failures

#### 4. High CPU/Memory Usage

**Symptoms**: Pods consuming excessive resources

**Solutions**:
```bash
# Check resource usage
kubectl top pods -n vantisos-production

# Check resource limits
kubectl describe deployment vantisos -n vantisos-production

# Adjust resource limits
kubectl set resources deployment vantisos -n vantisos-production \
  --limits=cpu=2000m,memory=2Gi \
  --requests=cpu=500m,memory=512Mi
```

**Common Causes**:
- Insufficient resource limits
- Memory leaks
- Inefficient code
- High traffic

### Debugging Commands

```bash
# Get pod details
kubectl describe pod <pod-name> -n vantisos-production

# Get pod logs
kubectl logs <pod-name> -n vantisos-production

# Get previous pod logs
kubectl logs <pod-name> -n vantisos-production --previous

# Execute command in pod
kubectl exec -it <pod-name> -n vantisos-production -- /bin/bash

# Port forward to pod
kubectl port-forward <pod-name> 8080:8080 -n vantisos-production

# Copy files to/from pod
kubectl cp local-file.txt <pod-name>:/path/to/file -n vantisos-production
kubectl cp <pod-name>:/path/to/file local-file.txt -n vantisos-production
```

### Monitoring and Alerts

```bash
# Check Prometheus targets
kubectl port-forward svc/prometheus-kube-prometheus-prometheus -n monitoring 9090:9090
# Open http://localhost:9090/targets

# Check Grafana dashboards
kubectl port-forward svc/grafana -n monitoring 3000:80
# Open http://localhost:3000

# Check alerts
kubectl port-forward svc/prometheus-kube-prometheus-alertmanager -n monitoring 9093:9093
# Open http://localhost:9093
```

---

## Best Practices

### 1. Use GitOps

- Store all configuration in Git
- Use ArgoCD for automated deployments
- Maintain complete deployment history
- Enable easy rollbacks

### 2. Implement Health Checks

```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10

readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 5
```

### 3. Use Resource Limits

```yaml
resources:
  requests:
    cpu: "500m"
    memory: "512Mi"
  limits:
    cpu: "2000m"
    memory: "2Gi"
```

### 4. Implement Autoscaling

```yaml
autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 10
  targetCPUUtilizationPercentage: 70
```

### 5. Use Secrets Management

```bash
# Use Kubernetes secrets
kubectl create secret generic vantisos-secrets \
  --from-literal=DATABASE_PASSWORD='password' \
  -n vantisos-production

# Use external secret management (AWS Secrets Manager, Azure Key Vault)
```

### 6. Implement Logging

```yaml
# Use centralized logging
# Logs are sent to ELK stack
# Configure log levels appropriately
```

### 7. Use Labels and Annotations

```yaml
metadata:
  labels:
    app: vantisos
    version: "0.4.1"
    environment: production
  annotations:
    prometheus.io/scrape: "true"
    prometheus.io/port: "8080"
```

---

## Conclusion

This deployment guide provides comprehensive instructions for deploying VantisOS to production environments. Follow the steps carefully and verify each stage before proceeding to the next.

**Key Points**:
- ✅ Ensure all prerequisites are met
- ✅ Follow the deployment process step by step
- ✅ Verify deployment success
- ✅ Monitor application health
- ✅ Have rollback plan ready
- ✅ Follow best practices

**Support**:
- Documentation: https://docs.vantisos.com
- Issues: https://github.com/vantisCorp/VantisOS/issues
- Support: support@vantisos.com

---

**Document Version**: 1.0  
**Last Updated**: February 26, 2025  
**Author**: VantisOS Infrastructure Team