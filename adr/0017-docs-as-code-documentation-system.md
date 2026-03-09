# ADR-0017: Docs-as-Code Documentation System

## Status

**Accepted**

## Context

Traditional documentation approaches:
- **Separate systems**: Documentation in Word, Google Docs, Confluence
- **Stale quickly**: Documentation lags behind code changes
- **No version control**: Hard to track changes
- **No review process**: Documentation quality varies
- **Difficult to test**: Cannot verify documentation accuracy

VantisOS requirements:
- **Always up-to-date**: Documentation must match code
- **Version controlled**: Track all changes
- **Review process**: Documentation goes through PR review
- **Automated**: Documentation generation automated
- **Testable**: Can verify documentation correctness

## Decision

VantisOS will adopt **Docs-as-Code** documentation system:

**Principles**:
1. **Documentation is code**: Documentation in version control with code
2. **Markdown/AsciiDoc**: Lightweight markup languages
3. **PR-based review**: Documentation reviewed like code
4. **Automated generation**: Generate HTML/PDF from source
5. **Continuous integration**: Documentation tested in CI/CD

**Documentation Structure**:
```
docs/
├── architecture/     # Architecture documentation
├── api/             # API documentation
├── guides/          # User and developer guides
├── reports/         # Reports and analyses
├── governance/      # Governance and community docs
└── adr/             # Architecture Decision Records
```

**Tools**:
- **AsciiDoctor**: Professional documentation generation
- **Hugo**: Static site generator for docs
- **Mermaid.js**: Diagrams in Markdown
- **Vale Linter**: Documentation style linter
- **IETF RFC 2119**: Compliance for requirements

**Automated Generation**:
- **API docs**: Generate from Rust doc comments
- **Diagrams**: Generate Mermaid diagrams from code
- **Cross-references**: Auto-link between code and docs
- **Versioning**: Multiple documentation versions

## Consequences

### Positive
- **Always up-to-date**: Documentation updated with code
- **Version controlled**: Full history of changes
- **Review process**: Documentation reviewed like code
- **Automated**: No manual documentation work
- **Testable**: Can verify documentation accuracy

### Negative
- **Learning curve**: Developers must learn documentation tools
- **Initial setup**: Infrastructure setup required
- **Overhead**: Every PR may need documentation update
- **Tooling complexity**: Multiple tools to learn

### Affected Systems
- All code (must have doc comments)
- Documentation (in version control)
- CI/CD pipeline (documentation generation)
- Development workflow (documentation PRs)

## Alternatives Considered

### Separate Documentation System
- **Pros**: Simple, familiar
- **Cons**: Stale quickly, no version control
- **Rejected**: Want documentation as code

### Manual Documentation
- **Pros**: No tooling overhead
- **Cons**: Not automated, hard to maintain
- **Rejected**: Want automation

### Third-Party Documentation Platforms
- **Pros**: No infrastructure
- **Cons**: External dependency, cost
- **Rejected**: Want control and automation

### Wiki-Based Documentation
- **Pros**: Easy to update
- **Cons**: No review process, quality varies
- **Rejected**: Want review process like code

## Related Decisions

- **ADR-0001**: Use Rust as primary language (doc comments)
- **ADR-0002**: Adopt microkernel architecture (architecture docs)

## References

- [Docs-as-Code](https://www.writethedocs.org/guide/docs-as-code/)
- [AsciiDoctor](https://asciidoctor.org/)
- [Hugo Static Site Generator](https://gohugo.io/)
- [IETF RFC 2119](https://tools.ietf.org/html/rfc2119)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented
- [ ] Automated generation

---

**Author**: VantisOS Team  
**Date**: 2024-09-01  
**Last Updated**: 2025-02-24