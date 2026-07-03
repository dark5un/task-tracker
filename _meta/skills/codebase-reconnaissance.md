# Codebase Reconnaissance Skill

A Hermes skill for systematic exploration of an unfamiliar codebase before any fix or feature work.

## Trigger

When assigned a bug, tech debt, or feature task in an existing codebase that has not been previously explored.

## Phases

### Phase 1: Surface Scan

1. Clone/fork the repo and note language(s), total LOC, file count, build system, test framework.
2. Run the existing test suite to establish the baseline.
3. Identify the target (issue number, bug description, feature request).

### Phase 2: Blast Radius

4. Trace the affected code path using LSP tools (find_references, explore_symbol, blast_radius).
5. Map the dependency graph: which packages/modules/tests depend on the affected code.
6. Document the root cause: where behavior diverges from expected contract.

### Phase 3: Knock-on Inventory

7. Write the repro test early and confirm it fails (RED phase).
8. Identify downstream breakage: which utility functions, composite types, or tests rely on the buggy behavior.
9. Produce the reconnaissance report.

### Phase 4: Handoff

10. Save the report for work item creation.
11. Pass findings to the work item composer skill.

## Output Format

```
## Reconnaissance Report

**Target:** [issue/bug description]
**Codebase:** [language, LOC, structure notes]
**Blast Radius:** [affected files, symbols, implementations]
**Root Cause:** [what actually causes the behavior]
**Downstream Impact:** [known knock-ons, their severity]
**Surprises:** [what contradicted initial assumptions]
**Recommended Fix Direction:** [which approach to take]
```

## References

See the full skill definition in Hermes: `skill_view(name='codebase-reconnaissance')`