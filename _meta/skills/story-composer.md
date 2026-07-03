# Work Item Composer Skill

Writes structured work items. Based on an iterative methodology: items describe work from the user's perspective, sliced small enough to complete in a single cycle.

## The Story Types

| Type | Description | States | Estimated? |
|------|-------------|--------|------------|
| Feature | Verifiable business value to the customer | unscheduled, unstarted, started, finished, delivered, accepted, rejected | Yes |
| Bug | Unintended behavior (feature that broke) | (same as Feature) | Optional |
| Maintenance | Necessary but no direct user value | unscheduled, started, accepted | No |
| Milestone | Marker for tracking progress | unscheduled, started, accepted | No |

## Story Structure

```
## Story: [one-line title]

**Type:** [Feature | Bug | Maintenance | Milestone]
**Theme:** [name of the epic this belongs to, if any]
**Labels:** [blast-radius, affected-components, language, lsp-quality]

**Description:**

A few sentences from the user's perspective.

**Acceptance Criteria:**

- [ ] [criterion 1]
- [ ] [criterion 2]

**Tasks:**

- [ ] [task 1]
- [ ] [task 2]
```

## Labels Convention

- `language:go`, `language:python`, `language:rust`
- `lsp:rust-analyzer`, `lsp:gopls`, `lsp:pyright`
- `codebase:greenfield`, `codebase:existing`
- `scope:tdd`, `scope:tech-debt`, `scope:bug`
- `experiment:n`

## References

Full skill definition: `skill_view(name='work item-composer')`