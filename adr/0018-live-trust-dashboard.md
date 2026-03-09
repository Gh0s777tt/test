# ADR-0018: Live Trust Dashboard for Real-Time Security Metrics

## Status

**Accepted**

## Context

Traditional OS security monitoring:
- **Static metrics**: Uptime, version numbers
- **Delayed reports**: Security reports after incidents
- **No visibility**: Cannot see current security state
- **Black box**: Don't know if security is working

VantisOS requirements:
- **Transparency**: Always show current security state
- **Real-time**: Live updates of security metrics
- **Actionable**: Clear indicators of security issues
- **Provable**: Show formal verification status live

## Decision

VantisOS will implement a **Live Trust Dashboard** for real-time security metrics:

**Dashboard Features**:
1. **Days without memory error**: Counter for memory-safety guarantees
2. **Formal verification progress**: Live status of Verus/Kani proofs
3. **Fuzzing metrics**: OSS-Fuzz results in real-time
4. **Security incidents**: Live count of security events
5. **System health**: CPU, memory, network health
6. **Compliance status**: EAL7+, FIPS 140-3, ISO 27001 progress

**Metrics Displayed**:
- **Uptime**: System uptime with error-free days
- **Verification coverage**: Percentage of code verified
- **Proofs passing**: Number of proofs passing verification
- **Bugs found**: Security bugs found by fuzzing
- **Vulnerabilities**: CVE count and severity
- **Compliance**: Certification progress and status

**Real-Time Updates**:
- **WebSocket**: Live updates via WebSocket
- **Pull interval**: Refresh every 5 seconds
- **Event-driven**: Immediate updates for security events
- **Historical trends**: Graphs showing trends over time

**Integration**:
- **README**: Embedded in project README
- **GitHub Actions**: Status checks in CI/CD
- **OSS-Fuzz**: Fuzzing results displayed
- **Verus/Kani**: Verification status live
- **Monitoring**: Integrated with system monitoring

## Consequences

### Positive
- **Transparency**: Everyone sees security state
- **Trust**: Provable security metrics
- **Accountability**: Issues are visible immediately
- **Motivation**: Live metrics motivate improvement
- **Marketing**: Unique selling point

### Negative
- **Complexity**: Dashboard infrastructure adds complexity
- **False positives**: May show issues that aren't critical
- **Overhead**: Continuous monitoring overhead
- **Privacy**: Some metrics may be sensitive

### Affected Systems
- README (embedded dashboard)
- CI/CD pipeline (status checks)
- Monitoring system (data collection)
- Verus/Kani (proof status)
- OSS-Fuzz (fuzzing results)

## Alternatives Considered

### Static Security Reports
- **Pros**: Simple, no overhead
- **Cons**: Not real-time, delayed
- **Rejected**: Want real-time visibility

### No Dashboard
- **Pros**: No complexity
- **Cons**: No transparency
- **Rejected**: Transparency is non-negotiable

### Internal Dashboard Only
- **Pros**: Less exposure
- **Cons**: No public trust
- **Rejected**: Want public transparency

### External Monitoring Service
- **Pros**: No infrastructure
- **Cons**: External dependency, cost
- **Rejected**: Want control and custom metrics

## Related Decisions

- **ADR-0005**: Formal verification with Verus/Kani
- **ADR-0015**: OSS-Fuzz integration
- **ADR-0017**: Docs-as-Code documentation

## References

- [GitHub Actions Status Badges](https://docs.github.com/en/actions/monitoring-and-troubleshooting-workflows/adding-a-workflow-status-badge)
- [Prometheus Monitoring](https://prometheus.io/)
- [Grafana Dashboards](https://grafana.com/)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [ ] Implemented
- [ ] Integrated with CI/CD

---

**Author**: VantisOS Team  
**Date**: 2024-09-15  
**Last Updated**: 2025-02-24