# Methodology

Our process for structured experiments on the TDD pipeline.

## Pipeline

1. **Codebase Reconnaissance** -- explore the target, trace blast radius, produce report
2. **Story Composition** -- write a PivotalTracker-style story from the report
3. **RED-GREEN-REFACTOR** -- execute the TDD cycle
4. **CONCLUSION.md** -- meditation on what worked, what did not, what surprised

## Directory Structure

```
methodology/
  README.md              -- this file
  skills/
    codebase-reconnaissance.md
    story-composer.md
  stories/               -- stories for each experiment
  reconnaissance/        -- reconnaissance reports
  references/            -- external reference material
```

## Experiment Register

| # | Target | Language | Scope | Status |
|---|--------|----------|-------|--------|
| 1 | spf13/afero (issue #270) | Go | Tech debt refactor | Done |
| 2 | tqdm/tqdm | Python | Tech debt refactor | Pending |
| 3 | Python (TDD from scratch) | Python | Greenfield TDD | Pending |
| 4 | Go (TDD from scratch) | Go | Greenfield TDD | Pending |

## The Pattern

Each experiment produces:
- A branch on the forked repo with the fix
- A story in methodology/stories/
- A reconnaissance report in methodology/reconnaissance/
- A CONCLUSION.md meditation