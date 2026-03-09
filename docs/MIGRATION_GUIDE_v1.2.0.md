# Migration Guide: VantisOS v1.2.0 "Cloud Native"

This guide helps you migrate from VantisOS v1.0.0/v1.1.0 to v1.2.0 with cloud-native capabilities.

## Table of Contents

1. [Overview](#overview)
2. [New Features](#new-features)
3. [Breaking Changes](#breaking-changes)
4. [Migration Steps](#migration-steps)
5. [Cloud Provider Setup](#cloud-provider-setup)
6. [Kubernetes Integration](#kubernetes-integration)
7. [Multi-Cloud Deployment](#multi-cloud-deployment)
8. [Troubleshooting](#troubleshooting)

---

## Overview

VantisOS v1.2.0 introduces comprehensive cloud-native capabilities including:
- Multi-cloud support (AWS, Azure, GCP)
- Kubernetes integration
- Distributed computing framework
- Cloud deployment automation
- Enhanced monitoring and security

### Version Compatibility

| From Version | To Version | Complexity | Estimated Time |
|--------------|------------|------------|----------------|
| v1.1.0       | v1.2.0     | Low        | 1-2 hours      |
| v1.0.0       | v1.2.0     | Medium     | 2-4 hours      |

---

## New Features

### 1. Multi-Cloud Support

#### AWS Integration
```rust
use vantis::verified::multicloud::aws::AwsClient;

let config = AwsConfig::new(
    "us-east-1",
    "access_key".to_string(),
    "secret_key".to_string(),
);

let client = AwsClient::new(config);
let instances = client.list_instances().await?;
```

#### Azure Integration
```rust
use vantis::verified::multicloud::azure::AzureClient;

let config = AzureConfig::new(
    "eastus",
    "subscription_id".to_string(),
    "client_id".to_string(),
    "client_secret".to_string(),
    "tenant_id".to_string(),
);

let client = AzureClient::new(config);
let vms = client.list_virtual_machines().await?;
```

#### GCP Integration
```rust
use vantis::verified::multicloud::gcp::GcpClient;

let config = GcpConfig::new(
    "us-central1",
    "project_id".to_string(),
    "service_account_key".to_string(),
);

let client = GcpClient::new(config);
let instances = client.list_instances().await?;
```

### 2. Kubernetes Integration

```rust
use vantis::verified::kubernetes::KubernetesClient;

// Create client from kubeconfig
let client = KubernetesClient::from_kubeconfig("~/.kube/config").await?;

// Or create in-cluster client
let client = KubernetesClient::in_cluster().await?;

// Deploy application
let deployment = client.create_deployment(deployment_spec).await?;
let service = client.create_service(service_spec).await?;
```

### 3. Multi-Cloud Abstraction

```rust
use vantis::verified::multicloud::abstract::MultiCloudManager;

let manager = MultiCloudManager::new();

// Add cloud providers
manager.add_provider("aws", Box::new(aws_provider));
manager.add_provider("azure", Box::new(azure_provider));
manager.add_provider("gcp", Box::new(gcp_provider));

// Deploy across multiple clouds
manager.deploy_multi_cloud("app-deployment", &deployment_config).await?;
```

---

## Breaking Changes

### 1. Module Reorganization

Several modules have been reorganized for better cloud-native support:

**Old Structure:**
```
src/verified/
├── cloud.rs
└── distributed/
```

**New Structure:**
```
src/verified/
├── cloud/
│   ├── deployment.rs
│   ├── autoscaling.rs
│   ├── loadbalancer.rs
│   └── service_mesh.rs
├── distributed/
│   ├── storage.rs
│   ├── cluster.rs
│   ├── ha.rs
│   └── disaster.rs
├── kubernetes/
│   ├── mod.rs
│   ├── client.rs
│   ├── config.rs
│   ├── pod.rs
│   └── ...
└── multicloud/
    ├── aws.rs
    ├── azure.rs
    ├── gcp.rs
    └── abstract.rs
```

### 2. API Changes

#### Cloud Configuration

**Old:**
```rust
let config = CloudConfig::new("region", "credentials");
```

**New:**
```rust
let config = AwsConfig::new("region", "access_key", "secret_key");
let azure_config = AzureConfig::new("region", "subscription_id", "client_id", "client_secret", "tenant_id");
let gcp_config = GcpConfig::new("region", "project_id", "service_account_key");
```

### 3. Dependency Updates

New dependencies have been added:

```toml
[dependencies]
# Cloud providers
aws-sdk-ec2 = "1.0"
azure-sdk = "1.0"
google-cloud-sdk = "1.0"

# Kubernetes
kube = "0.80"
k8s-openapi = "0.18"

# Distributed systems
libp2p = "0.50"
raft = "0.7"
```

---

## Migration Steps

### Step 1: Update Dependencies

Update your `Cargo.toml`:

```bash
cargo update
```

### Step 2: Update Module Imports

Replace old import paths with new ones:

```rust
// Old
use vantis::verified::cloud::CloudDeployment;

// New
use vantis::verified::cloud::deployment::CloudDeployment;
```

### Step 3: Configure Cloud Providers

Create cloud provider configuration files:

```yaml
# config/cloud_providers.yaml
aws:
  region: us-east-1
  access_key: ${AWS_ACCESS_KEY}
  secret_key: ${AWS_SECRET_KEY}

azure:
  region: eastus
  subscription_id: ${AZURE_SUBSCRIPTION_ID}
  client_id: ${AZURE_CLIENT_ID}
  client_secret: ${AZURE_CLIENT_SECRET}
  tenant_id: ${AZURE_TENANT_ID}

gcp:
  region: us-central1
  project_id: ${GCP_PROJECT_ID}
  service_account_key: ${GCP_SA_KEY}
```

### Step 4: Update Application Code

Update your application to use the new cloud-native APIs:

```rust
use vantis::verified::multicloud::abstract::MultiCloudManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize multi-cloud manager
    let mut manager = MultiCloudManager::new();
    
    // Configure providers
    manager.add_aws_provider(&aws_config)?;
    manager.add_azure_provider(&azure_config)?;
    manager.add_gcp_provider(&gcp_config)?;
    
    // Deploy application
    let deployment = manager.deploy_multi_cloud("my-app", &config).await?;
    
    Ok(())
}
```

### Step 5: Test Migration

Run tests to ensure everything works:

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration

# Test cloud connectivity
cargo test --test cloud_connectivity
```

---

## Cloud Provider Setup

### AWS Setup

1. **Create AWS Account**: Sign up at https://aws.amazon.com

2. **Create IAM User**:
   - Go to IAM console
   - Create new user with programmatic access
   - Attach appropriate policies (EC2, S3, VPC)

3. **Configure Credentials**:
   ```bash
   export AWS_ACCESS_KEY_ID="your_access_key"
   export AWS_SECRET_ACCESS_KEY="your_secret_key"
   export AWS_DEFAULT_REGION="us-east-1"
   ```

4. **Create VPC** (optional):
   ```rust
   let vpc = client.create_vpc("10.0.0.0/16").await?;
   ```

### Azure Setup

1. **Create Azure Account**: Sign up at https://azure.microsoft.com

2. **Create Service Principal**:
   ```bash
   az login
   az ad sp create-for-rbac --name "vantis-app"
   ```

3. **Configure Credentials**:
   ```bash
   export AZURE_SUBSCRIPTION_ID="your_subscription_id"
   export AZURE_CLIENT_ID="your_client_id"
   export AZURE_CLIENT_SECRET="your_client_secret"
   export AZURE_TENANT_ID="your_tenant_id"
   ```

### GCP Setup

1. **Create GCP Project**: Sign up at https://cloud.google.com

2. **Create Service Account**:
   ```bash
   gcloud iam service-accounts create vantis-app --display-name="Vantis App"
   gcloud iam service-accounts keys create key.json --iam-account=vantis-app@PROJECT_ID.iam.gserviceaccount.com
   ```

3. **Enable APIs**:
   ```bash
   gcloud services enable compute.googleapis.com
   gcloud services enable storage.googleapis.com
   gcloud services enable container.googleapis.com
   ```

---

## Kubernetes Integration

### Setting Up Kubernetes

1. **Create Kubernetes Cluster**:
   ```bash
   # AWS EKS
   eksctl create cluster --name vantis-cluster --region us-east-1
   
   # Azure AKS
   az aks create --resource-group vantis-rg --name vantis-cluster --node-count 3
   
   # GCP GKE
   gcloud container clusters create vantis-cluster --num-nodes 3
   ```

2. **Configure kubectl**:
   ```bash
   kubectl get nodes
   ```

3. **Deploy with VantisOS**:
   ```rust
   use vantis::verified::kubernetes::KubernetesClient;
   
   let client = KubernetesClient::from_kubeconfig("~/.kube/config").await?;
   
   // Create namespace
   client.create_namespace("vantis-app").await?;
   
   // Deploy application
   let deployment = client.create_deployment(&deployment_spec).await?;
   let service = client.create_service(&service_spec).await?;
   ```

### Example Deployment

```rust
use vantis::verified::kubernetes::pod::{PodSpec, Container};
use vantis::verified::kubernetes::deployment::{DeploymentSpec, DeploymentStrategy};

let pod_spec = PodSpec::new(vec![
    Container::new("app", "my-app:1.0.0")
        .with_port(8080)
        .with_env("DATABASE_URL", "postgresql://db:5432/app")
]);

let deployment_spec = DeploymentSpec::new("my-app", "vantis-app", pod_spec)
    .with_replicas(3)
    .with_strategy(DeploymentStrategy::RollingUpdate);

let deployment = client.create_deployment(deployment_spec).await?;
```

---

## Multi-Cloud Deployment

### Strategy Options

1. **Active-Active**: Run same workload on multiple clouds
2. **Active-Passive**: Primary in one cloud, backup in another
3. **Hybrid**: Different workloads in different clouds

### Example: Active-Active Deployment

```rust
use vantis::verified::multicloud::abstract::{MultiCloudManager, MultiCloudDeploymentConfig};

let config = MultiCloudDeploymentConfig::new()
    .with_strategy(MultiCloudStrategy::ActiveActive)
    .with_providers(vec!["aws", "azure", "gcp"])
    .with_regions(vec!["us-east-1", "eastus", "us-central1"])
    .with_replicas_per_provider(2);

let deployment = manager.deploy_multi_cloud("my-app", &config).await?;
```

### Load Balancing Across Clouds

```rust
use vantis::verified::cloud::loadbalancer::{LoadBalancer, LoadBalancingStrategy};

let lb = LoadBalancer::new()
    .with_strategy(LoadBalancingStrategy::Geographic)
    .with_backends(vec![
        "https://aws-app.example.com",
        "https://azure-app.example.com",
        "https://gcp-app.example.com",
    ]);

lb.deploy().await?;
```

---

## Troubleshooting

### Common Issues

#### 1. Authentication Errors

**Symptom**: `AuthenticationError: Invalid credentials`

**Solution**:
- Verify cloud provider credentials
- Check IAM permissions
- Ensure service account has correct roles

#### 2. Connection Timeouts

**Symptom**: `ConnectionError: Timeout connecting to cloud provider`

**Solution**:
- Check network connectivity
- Verify firewall rules
- Ensure cloud provider APIs are accessible

#### 3. Kubernetes API Errors

**Symptom**: `KubernetesError: API server unavailable`

**Solution**:
- Verify kubectl configuration
- Check Kubernetes cluster status
- Ensure RBAC permissions are correct

#### 4. Multi-Cloud Sync Issues

**Symptom**: `SyncError: Inconsistent state across clouds`

**Solution**:
- Check cloud provider status
- Verify network connectivity between clouds
- Review deployment logs

### Debug Mode

Enable debug logging:

```rust
use vantis::logger;

logger::set_level(log::LevelFilter::Debug);
```

### Getting Help

- **Documentation**: [Cloud Native Guide](CLOUD_NATIVE_GUIDE.md)
- **Examples**: [examples/](../examples/)
- **Issues**: [GitHub Issues](https://github.com/vantisCorp/VantisOS/issues)
- **Discord**: [VantisOS Citadel](https://discord.gg/dSxQXXVBhx)

---

## Next Steps

After migration:

1. **Explore Cloud Native Features**: Read the [Cloud Native Guide](CLOUD_NATIVE_GUIDE.md)
2. **Try Examples**: Check out the [examples directory](../examples/)
3. **Deploy to Cloud**: Follow the deployment guides
4. **Monitor Performance**: Set up cloud monitoring
5. **Optimize Costs**: Review and optimize cloud resource usage

---

## Additional Resources

- [Release Notes](RELEASE_NOTES_v1.2.0.md)
- [API Reference](API_REFERENCE.md)
- [Cloud Native Guide](CLOUD_NATIVE_GUIDE.md)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [AWS Documentation](https://docs.aws.amazon.com/)
- [Azure Documentation](https://docs.microsoft.com/azure/)
- [GCP Documentation](https://cloud.google.com/docs)

---

**Last Updated**: March 3, 2026  
**Version**: 1.2.0  
**Status**: Production Ready ✅