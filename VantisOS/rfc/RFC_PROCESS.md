# RFC (Request for Comments) Process

## Overview

The RFC (Request for Comments) process is how the VantisOS community makes major decisions about the project. RFCs are used for significant changes that affect the project's direction, architecture, or governance.

## When to Use RFCs

RFCs should be used for:

**Major architectural changes**:
- New subsystems or components
- Breaking changes to existing APIs
- Changes to the kernel architecture

**Governance changes**:
- Changes to decision-making processes
- New policies or procedures
- Changes to roles and responsibilities

**Major features**:
- Features that require significant design work
- Features that affect multiple components
- Features that have alternatives worth considering

**Partnerships and collaborations**:
- External partnerships
- Grant applications
- Funding proposals

**When NOT to use RFCs**:
- Bug fixes (use PRs)
- Minor features (use PRs)
- Documentation updates (use PRs)
- Routine maintenance (use PRs)

## RFC Process

### Phase 1: Draft RFC

1. **Create RFC**: Create a new RFC in `rfc/` directory
2. **Follow template**: Use `rfc/0000-template.md` as starting point
3. **Numbering**: Use next available number (e.g., RFC-0001)
4. **Submit PR**: Open PR with the RFC draft

### Phase 2: TSC Review (7 days)

1. **TSC review**: Technical Steering Committee reviews the RFC
2. **Initial feedback**: TSC provides initial feedback
3. **Revisions**: Author may revise RFC based on feedback
4. **Ready for discussion**: TSC marks RFC as "Ready for Discussion"

### Phase 3: Community Discussion (30 days)

1. **GitHub Discussion**: Create GitHub Discussion for RFC
2. **Community review**: Community reviews and discusses RFC
3. **Feedback collection**: Collect all feedback
4. **Revisions**: Author revises RFC based on community feedback

### Phase 4: TSC Deliberation (14 days)

1. **TSC deliberation**: TSC deliberates on RFC
2. **Decision**: TSC votes on RFC (requires 4/5 approval)
3. **Status update**: Update RFC status to Accepted/Rejected/Deferred

### Phase 5: Implementation

1. **Implementation planning**: Author creates implementation plan
2. **Tracking**: Track implementation in project management
3. **Completion**: Mark RFC as "Implemented" when complete

### Phase 6: Post-Implementation Review

1. **Retrospective**: Review implementation results
2. **Lessons learned**: Document lessons learned
3. **RFC update**: Update RFC with final results

## RFC Template Structure

```markdown
# RFC-[Number]: [Title]

## Status
[Proposed | Accepted | Rejected | Deferred]

## Author
[Name] (@GitHub)

## Summary
[Brief description]

## Motivation
[Why are we doing this?]

## Detailed Design
[Technical details]

## Drawbacks
[What are the downsides?]

## Rationale and Alternatives
[Why this design? What alternatives?]

## Prior Art
[What other projects?]

## Unresolved Questions
[What is still TBD?]

## Implementation Plan
[Timeline and phases]

## Testing
[How will this be tested?]

## Risks and Mitigations
[What are the risks?]

## Success Criteria
[How will we know it's successful?]

## Dependencies
[What does this depend on?]

## References
[Links to relevant info]
```

## TSC Review Criteria

The Technical Steering Committee evaluates RFCs based on:

**Technical merit**:
- Does the RFC solve a real problem?
- Is the technical sound?
- Is it feasible to implement?

**Alignment with project goals**:
- Does it align with VantisOS vision?
- Does it support formal verification goals?
- Does it maintain security standards?

**Impact**:
- What is the impact on existing code?
- What is the impact on users?
- What is the impact on the community?

**Alternatives**:
- Have alternatives been considered?
- Why is this the best approach?

**Completeness**:
- Is the RFC complete?
- Are all questions answered?
- Is the implementation plan clear?

## Community Review Guidelines

Community members should review RFCs and provide feedback on:

**Clarity**:
- Is the RFC clear and understandable?
- Are technical terms explained?

**Feasibility**:
- Is the RFC feasible to implement?
- Are resources available?

**Impact**:
- How will this affect you?
- What are the benefits and drawbacks?

**Suggestions**:
- Do you have suggestions for improvement?
- Have you seen similar work elsewhere?

**Be constructive**: Provide specific, actionable feedback

## Decision Process

### Acceptance

RFCs are accepted if:

1. **TSC approval**: 4/5 TSC members vote to approve
2. **Community support**: No major community objections
3. **Implementation resources**: Resources are available for implementation
4. **Clear plan**: Implementation plan is clear and achievable

### Rejection

RFCs may be rejected if:

1. **TSC rejection**: 3+ TSC members vote to reject
2. **Community opposition**: Strong community opposition
3. **Misalignment**: Doesn't align with project goals
4. **Not feasible**: Cannot be implemented with available resources

### Deferral

RFCs may be deferred if:

1. **Not ready now**: Good idea but not ready for implementation
2. **Requires more work**: Needs more research or design
3. **Timing**: Better timing in the future
4. **Dependencies**: Waiting on other RFCs or work

### Superseding

RFCs may be superseded if:

1. **Newer RFC**: A newer RFC supersedes this one
2. **Updated approach**: Better approach is proposed

## RFC Lifecycle

```
Draft → TSC Review → Community Discussion → TSC Decision → Implementation → Implemented
           ↓                ↓                  ↓
        Revision       Revision           Deferred
```

## Best Practices

### For Authors

**Before writing RFC**:
- Discuss idea in community first
- Get initial feedback
- Review similar RFCs

**Writing RFC**:
- Be clear and concise
- Provide context and motivation
- Consider alternatives
- Include implementation plan

**During review**:
- Respond to feedback promptly
- Be open to suggestions
- Update RFC based on feedback
- Be respectful of reviewers

### For Reviewers

**Reviewing RFCs**:
- Read the entire RFC carefully
- Provide specific feedback
- Be constructive
- Consider the bigger picture

**During discussion**:
- Focus on technical merits
- Be respectful of authors
- Consider alternatives
- Help improve the RFC

## RFC Examples

### Example: RFC-0001 - Adopt Microkernel Architecture

**Status**: Accepted

**Summary**: Adopt microkernel architecture for VantisOS

**Motivation**: Improve security, formal verification, fault isolation

**Key Design**: Minimal kernel, user-space services, capability-based IPC

**Result**: Implemented (see ADR-0002)

### Example: RFC-0002 - Reject POSIX Compliance

**Status**: Accepted

**Summary**: Consciously reject POSIX compliance for VantisOS

**Motivation**: POSIX has design flaws, limits innovation

**Key Design**: Custom APIs, no compatibility layer

**Result**: Implemented (see ADR-0003)

## RFC Directory Structure

```
rfc/
├── 0000-template.md
├── 0001-adopt-microkernel.md
├── 0002-reject-posix.md
├── 0003-webassembly-apps.md
├── RFC_PROCESS.md
└── README.md
```

## Tracking RFCs

All RFCs are tracked in:

1. **GitHub Discussions**: Each RFC has a discussion thread
2. **GitHub Issues**: RFCs are tracked as issues for implementation
3. **Project Management**: RFCs are tracked in project board
4. **README**: List of all RFCs with status

## Updating RFCs

RFCs can be updated:

**During review**: Anytime during review process
**After acceptance**: Minor clarifications only
**Major changes**: New RFC required

## RFC vs ADR

**RFCs (Request for Comments)**:
- Used for major decisions
- Requires community review
- Future-looking (what should we do?)
- Status: Proposed/Accepted/Rejected

**ADRs (Architecture Decision Records)**:
- Document decisions made
- Past-looking (what did we do?)
- No review required (already decided)
- Status: Accepted/Deprecated/Superseded

**Relationship**:
- RFCs are proposed changes
- Accepted RFCs become ADRs
- ADRs record the final decision

## Questions?

For questions about the RFC process:

- **Email**: governance@vantisos.org
- **GitHub Discussions**: #rfc channel
- **GitHub Issues**: Label with `rfc`

---

**Version**: 1.0  
**Created**: February 24, 2025  
**Last Updated**: February 24, 2025