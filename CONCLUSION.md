# Meditation: Git Traceability for Agent-Driven Development

**Date:** July 2, 2026
**Project:** task-tracker CLI
**Master:** @dark5un
**Padawan:** Hermes Agent

## The Pattern

This experiment validated a methodology for making AI agent code development reviewable and traceable:

1. **Brainstorm** — clarify scope, define behaviors, plan feature branches
2. **Branch per feature** — `git checkout -b feature/<name>`
3. **TDD per cycle** — RED → LSP → GREEN → LSP → REFACTOR → LSP
4. **Commit per cycle** — `git commit -m "Cycle N: <behavior>"`
5. **Squash into main** — `git merge --squash` for one clean feature commit
6. **Tag the trace** — `git tag trace/<name>` before deleting the branch
7. **Publish** — push to GitHub with tags
8. **Meditate** — reflect on what worked, what didn't, what surprised

## What Worked

- **The two-view history:** `git log main` for humans, `git log trace/<name>` for forensic traceability
- **Feature branch isolation:** zero conflicts across 4 sequential branches
- **Per-cycle commits:** each commit is a reviewable unit of work
- **Tags survive branch deletion:** the trace is preserved as a first-class git object
- **`git push --tags`** makes traces visible on GitHub

## What Didn't

- **Batching tests breaks the trace** — one test per edit is slower but cleaner
- **Path dependencies don't travel** — `../mini-clap` works locally but breaks for cloners
- **Worktrees untested** — parallel agent model is theoretical, not exercised
- **`git push --tags` is easy to forget** — must be explicit in Phase 5.6

## What Surprised

Agent work is inscrutable by default. The trace tags make it verifiable. A reviewer can `git log trace/add-task` and see: "they wrote the test first, then implemented, then cleaned up." That's the difference between trusting the output and verifying the reasoning.

## Git Logs

```
main (clean):
  feat: JSON persistence for task store
  feat: mark tasks complete by index
  feat: list tasks from store
  feat: add task creation
  Initial scaffolding

trace/add-task (per-cycle):
  Cycle 2: add subcommand
  Cycle 1: Task model
  Initial scaffolding
```

## Stats

- 4 feature branches, 8 TDD cycles
- 8 passing tests, 92% coverage
- 4 trace tags pushed to GitHub
---

## Meditation: Local Inference Validation

**Date:** July 2, 2026 (evening)
**Experiment:** Running the TDD pipeline on local hardware via ds4 (DwarfStar)
**Purpose:** Prove that real development work can run on your own machine. The cloud is for speed, not dependency.
**Hardware:** M2 Max, 96GB, SSD streaming, 200K context
**Model:** DeepSeek V4 Flash -- insanely cheap, does the job I need
**Session:** 20260702_194154_d74b01 (1h 15m, 75 tool calls)
**GitHub:** dark5un/task-tracker, trace/complete-task tag

### The Pattern

1. Set up ds4-server on the Mac with `--ctx 200000 --ssd-streaming`
2. Clone task-tracker, reset to state before complete-task feature
3. Configure Hermes to use ds4 as a provider
4. Load the agent-workflow-pipelines skill
5. Paste the task brief -- implement complete-task via TDD
6. Compare the output to the original cloud run

### What Worked

- The pipeline reproduced identically on local hardware. Three clean cycles, seven passing tests, a trace tag on GitHub.
- The local model followed the methodology more strictly than the cloud version. Three per-cycle commits instead of one bundled commit.
- Confirm-only cycles were documented honestly instead of forcing fake RED-GREEN loops.
- DwarfStar handled a 40K context window gracefully on 96GB with SSD streaming.
- GPU power consumption was 14.3W at 93% utilization.

### What Did Not

- Decoding speed was 8-11 t/s.
- The first session took 1 hour 15 minutes. Patience is required.
- One stream write error occurred (KV cache miss) but the server recovered automatically.
- Setup friction: mini-clap path dependency broke on fresh clone. Fixed with git URL.
- Without `--ssd-streaming`, the model fails to initialize on 96GB. The MoE routed experts are too large for RAM alone.

### What Surprised

Running the same model locally produced cleaner results than the cloud version. I expected degradation. I got improvement. The slower pace may have forced tighter cycle discipline. There was no incentive to bundle commits because there was no impatience to move faster.

The methodology is model-agnostic. It was validated on OpenRouter (cloud) and DwarfStar (local Metal) with identical TDD outcomes. The pipeline does not care where the inference runs. It only cares that the rules are followed.

### Stats

| Metric | Value |
|--------|-------|
| Cycles | 3 clean cycles |
| Tests | 7 passed |
| GPU power | 14.3W |
| Speed | 8-11 t/s |
| Session time | 21 minutes |
| Cost | Electricity only |
| Commit discipline | Strict per-cycle |
