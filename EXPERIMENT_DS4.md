# Experiment: Local DeepSeek V4 Flash (ds4) TDD Pipeline

## Goal

Prove that the TDD pipeline with git traceability is model-agnostic and infrastructure-independent. A local DeepSeek V4 Flash instance running via ds4 (DwarfStar) on an M2 Max with 96GB RAM should reproduce the same quality of work we achieved via OpenRouter.

## Architecture

```
OrsonRius (OpenRouter)         Subagent (local ds4)
┌──────────────────────┐       ┌──────────────────────────┐
│ Orchestrator          │──────▶│ Worker                    │
│ Plans the experiment  │       │ Has: pipeline skill,      │
│ Writes task brief     │       │ TDD rules, project repo,  │
│ Verifies output       │       │ git trace instructions    │
│ Compares results      │       │ Runs RED-GREEN-REFACTOR   │
└──────────────────────┘       │ Commits per cycle          │
                               │ Squashes and tags          │
                               └──────────────────────────┘
```

## Phase 1: Connection Setup (waiting for Master)

1. Panos provides ds4-server connection details (host:port) via Telegram
2. Configure Hermes custom provider:
   - Provider name: ds4-local
   - Base URL: http://<host>:<port>/v1
   - API key: (none or as provided)
   - Model: deepseek-v4-flash (or whatever ds4 exposes)
3. Verify connection with a test completion

## Phase 2: Experiment Task

### Project: task-tracker (existing repo)

Clone the task-tracker repo to the local machine. Reset to the state BEFORE the complete-task feature was added. This means:

- main should have only: feat: add task creation, feat: list tasks
- The subagent must implement `complete-task` from scratch

### Task Brief (given to the subagent)

```
You are a coding agent running on a local DeepSeek V4 Flash instance.
Follow the agent-workflow-pipelines skill strictly.

Your task: Add the "complete task" feature to the task-tracker CLI.

Setup:
- Project is at <path>/task-tracker on main with two features merged
- Dependencies: mini-clap (local path), serde, serde_json
- Tests: cargo test
- LSP: available via cargo check

Phase 3c:
  git checkout -b feature/complete-task

Phase 5 - TDD Cycles:

  Cycle A (RED): Write a test that marks a task complete by index.
    Task is found, done flips to true.
    (GREEN: implement TaskStore::mark_done(index))

  Cycle B (RED): Write a test that returns an error for out-of-bounds index.
    (GREEN: add error handling)

  Cycle C (RED): Write a test that parses `task-tracker done <index>` via mini-clap.
    (GREEN: wire up the done subcommand)

  After each GREEN + full suite pass:
    git add -A && git commit -m "Cycle N: <description>"

Phase 5.5:
  git checkout main
  git merge --squash feature/complete-task
  git commit -m "feat: mark tasks complete by index with error handling"
  git tag trace/complete-task feature/complete-task
  git push origin --tags
  git branch -D feature/complete-task

CRITICAL RULES:
- Never write production code before a failing test.
- Each cycle is one RED-GREEN-VERIFY-COMMIT loop.
- Run cargo test (full suite) after every GREEN.
- LSP check after every write: cargo check. Blockers before test.
- Bundling: if two behaviors are tightly coupled, use Cycle 5-6 naming.
- Abandoned cycles: commit with "ABANDONED: reason" instead of leaving a gap.
```

## Phase 3: Evaluation

Compare the subagent's output to our original feature/complete-task:

| Criterion | Original (OpenRouter) | Local (ds4) |
|-----------|----------------------|-------------|
| Number of cycles | 1 (Cycle 5-6 bundled) | ? |
| Per-cycle commits | Yes | ? |
| Tests pass | Yes | ? |
| Full suite green | Yes | ? |
| Trace tag created | trace/complete-task | ? |
| Correct commit naming | Cycle 5-6 | ? |
| Code quality | Production | ? |

The key hypothesis: the local model may need more cycles (more iterations to get GREEN) but the pipeline should still deliver working, tested, traceable code. The guardrails (LSP, test suite) compensate for any reduction in model quality.

## Phase 4: Meditation (CONCLUSION.md)

After the experiment, write a CONCLUSION.md covering:
- The Pattern (what we did and why)
- What Worked (did the local model follow the pipeline?)
- What Did Not (where did it struggle?)
- What Surprised (unexpected results)
- Comparison table of local vs cloud
- Lessons for running agents on local hardware