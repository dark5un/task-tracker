# Story Composer Skill

Writes PivotalTracker-style stories. Based on the Pivotal Labs methodology: stories describe work from the user's perspective, sliced small enough to complete in a single cycle.

## The Four Story Types

| Type | Description | States | Estimated? |
|------|-------------|--------|------------|
| Feature | Verifiable business value to the customer | unscheduled, unstarted, started, finished, delivered, accepted, rejected | Yes |
| Bug | Unintended behavior (feature that broke) | (same as Feature) | Optional |
| Chore | Necessary but no direct customer value | unscheduled, unstarted, started, accepted | No |
| Release | Milestone marker for tracking progress | unscheduled, started, accepted | No |

## Story Structure

```
## Story: [one-line title]

**Type:** [Feature | Bug | Chore | Release]
**Epic:** [name of the epic this belongs to, if any]
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

PivotalTracker documentation archived at github.com/dark5un/pt-archive
Full skill definition: `skill_view(name='story-composer')`