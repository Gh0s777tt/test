# VantisOS Monitoring and Observability Guide

## Executive Summary

This document provides comprehensive guidance for monitoring and observability of VantisOS infrastructure and applications, including metrics collection, alerting, and troubleshooting.

**Version**: 1.0  
**Last Updated**: February 26, 2025  
**Monitoring Stack**: Prometheus, Grafana, Alertmanager, ELK Stack  

---

## Table of Contents

1. [Overview](#overview)
2. [Monitoring Architecture](#monitoring-architecture)
3. [Metrics Collection](#metrics-collection)
4. [Alerting](#alerting)
5. [Dashboards](#dashboards)
6. [Logging](#logging)
7. [Tracing](#tracing)
8. [Troubleshooting](#troubleshooting)

---

## Overview

### Monitoring Philosophy

VantisOS follows a comprehensive monitoring approach that ensures:

1. **Full Observability**: Complete visibility into system behavior
2. **Proactive Alerting**: Detect issues before they impact users
3. **Rapid Troubleshooting**: Quick identification and resolution of issues
4. **Performance Optimization**: Continuous performance monitoring and optimization
5. **Capacity Planning**: Proactive capacity planning based on trends

### Monitoring Pillars

```
┌─────────────────────────────────────────────────────────────┐
│                  Monitoring Pillars                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Metrics (What)                                     │   │
│  │  - System metrics (CPU, memory, disk)               │   │
│  │  - Application metrics (requests, errors, latency)  │   │
│  │  - Business metrics (users, revenue)                │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Logs (Why)                                         │   │
│  │  - Application logs                                 │   │
│  │  - System logs                                      │   │
│  │  - Audit logs                                       │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Traces (Where)                                     │   │
│  │  - Distributed tracing                              │   │
│  │  - Request flow                                     │   │
│  │  - Service dependencies                             │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Alerts (When)                                      │   │
│  │  - Threshold-based alerts                           │   │
│  │  - Anomaly detection                                │   │
│  │  - Predictive alerts                                │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Monitoring Architecture

### Components

```
┌─────────────────────────────────────────────────────────────┐
│              Monitoring Architecture                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Data Collection Layer                              │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐              │   │
│  │  │Prometheus│ │Node     │ │cAdvisor │              │   │
│  │  │         │ │Exporter │ │         │              │   │
│  │  └─────────┘  └─────────┘  └─────────┘              │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐              │   │
│  │  │Custom   │ │JMX      │ │Blackbox │              │   │
│  │  │Exporter │ │Exporter │ │Exporter │              │   │
│  │  └─────────┘  └─────────┘  └─────────┘              │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Storage and Processing Layer                       │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐              │   │
│  │  │Prometheus│ │Thanos   │ │Victoria │              │   │
│  │  │TSDB     │ │(Long-term│ │Metrics  │              │   │
│  │  │         │ │storage) │ │         │              │   │
│  │  └─────────┘  └─────────┘  └─────────┘              │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Alerting Layer                                     │   │
│  │  ┌─────────┐                                        │   │
│  │  │Alert    │                                        │   │
│  │  │Manager  │                                        │   │
│  │  └─────────┘                                        │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Visualization Layer                                │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐              │   │
│  │  │Grafana  │ │Kibana   │ │Jaeger   │              │   │
│  │  │         │ │         │ │UI       │              │   │
│  │  └─────────┘  └─────────┘  └─────────┘              │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Metrics Collection | Prometheus | Time-series database |
| Node Metrics | Node Exporter | System-level metrics |
| Container Metrics | cAdvisor | Container-level metrics |
| Application Metrics | Custom Exporters | Application-specific metrics |
| Long-term Storage | Thanos | Long-term metrics storage |
| Alerting | Alertmanager | Alert routing and management |
| Visualization | Grafana | Dashboards and visualization |
| Logging | ELK Stack | Log aggregation and analysis |
| Tracing | Jaeger | Distributed tracing |

---

## Metrics Collection

### System Metrics

#### CPU Metrics

| Metric | Description | Threshold |
|--------|-------------|-----------|
| `node_cpu_seconds_total` | Total CPU seconds | - |
| `node_cpu_usage_percentage` | CPU usage percentage | < 70% |
| `process_cpu_seconds_total` | Process CPU time | - |

#### Memory Metrics

| Metric | Description | Threshold |
|--------|-------------|-----------|
| `node_memory_MemAvailable_bytes` | Available memory | > 1 GB |
| `node_memory_usage_percentage` | Memory usage percentage | < 80% |
| `process_resident_memory_bytes` | Process memory usage | < 2 GB |

#### Disk Metrics

| Metric | Description | Threshold |
|--------|-------------|-----------|
| `node_filesystem_avail_bytes` | Available disk space | > 10 GB |
| `node_filesystem_size_bytes` | Total disk space | - |
| `node_disk_io_time_seconds_total` | Disk I/O time | < 100 ms |

#### Network Metrics

| Metric | Description | Threshold |
|--------|-------------|-----------|
| `node_network_receive_bytes_total` | Total bytes received | - |
| `node_network_transmit_bytes_total` | Total bytes transmitted | - |
| `node_network_up` | Network interface status | 1 |

### Application Metrics

#### HTTP Metrics

| Metric | Description | Threshold |
|--------|-------------|-----------|
| `http_requests_total` | Total HTTP requests | - |
| `http_request_duration_seconds` | Request duration | < 100ms (p95) |
| `http_requests_in_flight` | In-flight requests | < 1000 |

#### Error Metrics

| Metric | Description | Threshold |
|--------|-------------|-----------|
| `http_requests_total{status=~"5.."}` | 5xx errors | < 0.1% |
| `http_requests_total{status=~"4.."}` | 4xx errors | < 1% |
| `application_errors_total` | Application errors | < 10/min |

#### Business Metrics

| Metric | Description | Threshold |
|--------|-------------|-----------|
| `active_users_total` | Active users | < 10,000 |
| `transactions_total` | Total transactions | - |
| `revenue_total` | Total revenue | - |

### Custom Metrics

#### Application-Specific Metrics

```rust
// Custom metrics example
use prometheus::{Counter, Histogram, Registry};

// Request counter
let request_counter = Counter::new(
    "vantisos_requests_total",
    "Total number of requests"
).unwrap();

// Request duration histogram
let request_duration = Histogram::new(
    "vantisos_request_duration_seconds",
    "Request duration in seconds"
).unwrap();

// Active users gauge
let active_users = Gauge::new(
    "vantisos_active_users",
    "Number of active users"
).unwrap();

// Register metrics
let registry = Registry::new();
registry.register(Box::new(request_counter.clone())).unwrap();
registry.register(Box::new(request_duration.clone())).unwrap();
registry.register(Box::new(active_users.clone())).unwrap();
```

---

## Alerting

### Alert Rules

#### Critical Alerts

```yaml
# infrastructure/monitoring/alerts/critical.yml
groups:
  - name: critical_alerts
    interval: 30s
    rules:
      # Service Down
      - alert: ServiceDown
        expr: up{job="vantisos"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Service {{ $labels.job }} is down"
          description: "Service {{ $labels.job }} has been down for more than 1 minute."

      # High Error Rate
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.01
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }} errors per second."

      # Database Down
      - alert: DatabaseDown
        expr: up{job="postgres"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Database is down"
          description: "Database has been down for more than 1 minute."
```

#### Warning Alerts

```yaml
# infrastructure/monitoring/alerts/warning.yml
groups:
  - name: warning_alerts
    interval: 1m
    rules:
      # High CPU Usage
      - alert: HighCPUUsage
        expr: cpu_usage_percentage > 70
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High CPU usage detected"
          description: "CPU usage is {{ $value }}%."

      # High Memory Usage
      - alert: HighMemoryUsage
        expr: memory_usage_percentage > 80
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage detected"
          description: "Memory usage is {{ $value }}%."

      # Disk Space Low
      - alert: DiskSpaceLow
        expr: disk_available_bytes < 10737418240
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Disk space is low"
          description: "Available disk space is {{ $value }} bytes."
```

#### Info Alerts

```yaml
# infrastructure/monitoring/alerts/info.yml
groups:
  - name: info_alerts
    interval: 5m
    rules:
      # Deployment Completed
      - alert: DeploymentCompleted
        expr: deployment_status == "completed"
        labels:
          severity: info
        annotations:
          summary: "Deployment completed"
          description: "Deployment {{ $labels.deployment }} completed successfully."

      # Backup Completed
      - alert: BackupCompleted
        expr: backup_status == "completed"
        labels:
          severity: info
        annotations:
          summary: "Backup completed"
          description: "Backup {{ $labels.backup }} completed successfully."
```

### Alert Routing

```yaml
# infrastructure/monitoring/alertmanager.yml
global:
  resolve_timeout: 5m

route:
  group_by: ['alertname', 'severity']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
  receiver: 'default'

  routes:
    # Critical alerts go to PagerDuty
    - match:
        severity: critical
      receiver: 'pagerduty'
      continue: false

    # Warning alerts go to Slack
    - match:
        severity: warning
      receiver: 'slack'
      continue: false

    # Info alerts go to email
    - match:
        severity: info
      receiver: 'email'
      continue: false

receivers:
  - name: 'default'
    email_configs:
      - to: 'alerts@vantisos.com'

  - name: 'pagerduty'
    pagerduty_configs:
      - service_key: '<PAGERDUTY_SERVICE_KEY>'

  - name: 'slack'
    slack_configs:
      - api_url: '<SLACK_WEBHOOK_URL>'
        channel: '#alerts'

  - name: 'email'
    email_configs:
      - to: 'alerts@vantisos.com'
        headers:
          Subject: '[VantisOS Alert] {{ .GroupLabels.alertname }}'
```

---

## Dashboards

### System Dashboard

```json
{
  "dashboard": {
    "title": "VantisOS System Overview",
    "panels": [
      {
        "title": "CPU Usage",
        "targets": [
          {
            "expr": "cpu_usage_percentage"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "targets": [
          {
            "expr": "memory_usage_percentage"
          }
        ]
      },
      {
        "title": "Disk Usage",
        "targets": [
          {
            "expr": "disk_usage_percentage"
          }
        ]
      },
      {
        "title": "Network Traffic",
        "targets": [
          {
            "expr": "rate(node_network_receive_bytes_total[5m])"
          }
        ]
      }
    ]
  }
}
```

### Application Dashboard

```json
{
  "dashboard": {
    "title": "VantisOS Application Metrics",
    "panels": [
      {
        "title": "Request Rate",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])"
          }
        ]
      },
      {
        "title": "Response Time",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, http_request_duration_seconds)"
          }
        ]
      },
      {
        "title": "Error Rate",
        "targets": [
          {
            "expr": "rate(http_requests_total{status=~&quot;5..&quot;}[5m])"
          }
        ]
      },
      {
        "title": "Active Users",
        "targets": [
          {
            "expr": "active_users_total"
          }
        ]
      }
    ]
  }
}
```

### Business Dashboard

```json
{
  "dashboard": {
    "title": "VantisOS Business Metrics",
    "panels": [
      {
        "title": "Revenue",
        "targets": [
          {
            "expr": "revenue_total"
          }
        ]
      },
      {
        "title": "Transactions",
        "targets": [
          {
            "expr": "rate(transactions_total[5m])"
          }
        ]
      },
      {
        "title": "Conversion Rate",
        "targets": [
          {
            "expr": "rate(transactions_total[5m]) / rate(http_requests_total[5m])"
          }
        ]
      }
    ]
  }
}
```

---

## Logging

### Log Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Log Architecture                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Log Sources                                        │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐              │   │
│  │  │Application│ │System   │ │Audit    │              │   │
│  │  │Logs     │ │Logs     │ │Logs     │              │   │
│  │  └─────────┘  └─────────┘  └─────────┘              │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Log Collection                                    │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐              │   │
│  │  │Filebeat │ │Fluentd  │ │Logstash │              │   │
│  │  └─────────┘  └─────────┘  └─────────┘              │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Log Storage and Indexing                          │   │
│  │  ┌─────────┐                                        │   │
│  │  │Elasticsearch│                                     │   │
│  │  └─────────┘                                        │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Log Visualization                                 │   │
│  │  ┌─────────┐                                        │   │
│  │  │Kibana   │                                        │   │
│  │  └─────────┘                                        │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Log Levels

| Level | Description | Usage |
|-------|-------------|-------|
| ERROR | Error conditions | Errors that need attention |
| WARN | Warning conditions | Potential issues |
| INFO | Informational messages | Normal operation |
| DEBUG | Debug messages | Detailed debugging information |
| TRACE | Trace messages | Very detailed tracing |

### Log Format

```json
{
  "timestamp": "2025-02-26T00:00:00Z",
  "level": "INFO",
  "service": "vantisos-api",
  "environment": "production",
  "message": "Request received",
  "request_id": "abc123",
  "user_id": "user123",
  "method": "GET",
  "path": "/api/v1/status",
  "status_code": 200,
  "duration_ms": 45
}
```

### Log Queries

#### Kibana Query Examples

```bash
# Search for errors
level:ERROR

# Search for specific service
service:vantisos-api

# Search for time range
@timestamp:[2025-02-26 TO 2025-02-27]

# Search for specific user
user_id:user123

# Search for slow requests
duration_ms:>1000

# Search for 5xx errors
status_code:[500 TO 599]
```

---

## Tracing

### Distributed Tracing

```
┌─────────────────────────────────────────────────────────────┐
│              Distributed Tracing Flow                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Client Request                                            │
│       │                                                     │
│       ▼                                                     │
│  ┌─────────┐                                               │
│  │ API     │ ────► Span 1: API Request                    │
│  │ Gateway │                                               │
│  └─────────┘                                               │
│       │                                                     │
│       ▼                                                     │
│  ┌─────────┐                                               │
│  │ API     │ ────► Span 2: API Processing                 │
│  │ Service │                                               │
│  └─────────┘                                               │
│       │                                                     │
│       ├──────────┬──────────┐                              │
│       ▼          ▼          ▼                              │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐                      │
│  │Database │ │Cache    │ │External │                      │
│  │         │ │         │ │Service  │                      │
│  └─────────┘ └─────────┘ └─────────┘                      │
│       │          │          │                              │
│       ▼          ▼          ▼                              │
│  Span 3     Span 4     Span 5                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Jaeger Integration

```rust
// Jaeger tracing example
use opentelemetry::trace::TraceError;
use opentelemetry_jaeger::{Exporter, Pipeline};
use opentelemetry::sdk::trace as sdktrace;

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    let exporter = Exporter::builder()
        .with_agent_endpoint("jaeger:6831".parse().unwrap())
        .with_process(opentelemetry_jaeger::Process {
            service_name: "vantisos-api".to_string(),
            tags: vec![],
        })
        .build()?;

    Ok(sdktrace::Tracer::builder()
        .with_exporter(exporter)
        .build())
}
```

---

## Troubleshooting

### Common Issues

#### 1. High CPU Usage

**Symptoms**: CPU usage above 70%

**Investigation**:
```bash
# Check CPU usage
kubectl top nodes
kubectl top pods -n vantisos-production

# Check CPU metrics
curl http://prometheus:9090/api/v1/query?query=cpu_usage_percentage

# Check processes
kubectl exec -it <pod-name> -n vantisos-production -- top
```

**Resolution**:
- Scale up resources
- Optimize application code
- Implement caching
- Use autoscaling

#### 2. High Memory Usage

**Symptoms**: Memory usage above 80%

**Investigation**:
```bash
# Check memory usage
kubectl top pods -n vantisos-production

# Check memory metrics
curl http://prometheus:9090/api/v1/query?query=memory_usage_percentage

# Check memory leaks
kubectl exec -it <pod-name> -n vantisos-production -- free -h
```

**Resolution**:
- Increase memory limits
- Fix memory leaks
- Optimize memory usage
- Implement memory limits

#### 3. High Error Rate

**Symptoms**: Error rate above 0.1%

**Investigation**:
```bash
# Check error logs
kubectl logs -f deployment/vantisos -n vantisos-production | grep ERROR

# Check error metrics
curl http://prometheus:9090/api/v1/query?query=rate(http_requests_total{status=~"5.."}[5m])

# Check application health
curl https://api.vantisos.com/health
```

**Resolution**:
- Fix application errors
- Implement retry logic
- Add circuit breakers
- Improve error handling

#### 4. Slow Response Time

**Symptoms**: Response time above 100ms (p95)

**Investigation**:
```bash
# Check response time metrics
curl http://prometheus:9090/api/v1/query?query=histogram_quantile(0.95, http_request_duration_seconds)

# Check traces
curl http://jaeger:16686/api/traces?service=vantisos-api

# Check database queries
kubectl logs -f deployment/vantisos -n vantisos-production | grep "slow query"
```

**Resolution**:
- Optimize database queries
- Implement caching
- Use CDN
- Optimize application code

---

## Best Practices

### 1. Use Meaningful Metrics

- Choose metrics that matter
- Avoid metric explosion
- Use labels appropriately
- Document metric definitions

### 2. Set Appropriate Thresholds

- Base thresholds on SLAs
- Use percentiles (p95, p99)
- Consider seasonal variations
- Review thresholds regularly

### 3. Implement Alert Fatigue Prevention

- Use alert severity levels
- Implement alert grouping
- Use alert silencing
- Review alert rules regularly

### 4. Maintain Dashboards

- Keep dashboards up to date
- Use consistent naming
- Document dashboard purpose
- Review dashboards regularly

### 5. Use Structured Logging

- Use JSON format
- Include context information
- Use appropriate log levels
- Avoid sensitive data in logs

---

## Conclusion

This monitoring and observability guide provides comprehensive guidance for monitoring VantisOS infrastructure and applications. Follow the best practices and regularly review and update monitoring configurations.

**Key Points**:
- ✅ Comprehensive monitoring architecture
- ✅ Metrics collection and alerting
- ✅ Dashboards for visualization
- ✅ Logging and tracing
- ✅ Troubleshooting procedures
- ✅ Best practices

**Next Steps**:
1. Implement monitoring stack
2. Configure alert rules
3. Create dashboards
4. Set up logging
5. Implement tracing
6. Test monitoring and alerting

---

**Document Version**: 1.0  
**Last Updated**: February 26, 2025  
**Author**: VantisOS Infrastructure Team