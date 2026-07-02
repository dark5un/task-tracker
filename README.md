# task-tracker

A simple CLI task tracker in Rust. Built to validate the git traceability methodology for agent-driven development.

Uses mini-clap for argument parsing, serde for JSON persistence.

## Features

- Add tasks with descriptions
- List all tasks with completion status
- Mark tasks as complete by index
- JSON persistence (save/load)
- Formatted display with status indicators

## Usage

```bash
# Add a task
task-tracker add "Buy milk"

# List all tasks
task-tracker list

# Mark a task as complete
task-tracker done 1
```

## How it was built

Four feature branches, eight TDD cycles, 92 percent coverage. Each feature was developed on its own branch, then squash-merged into main with a trace tag. The trace tags preserve the full per-cycle development history.

Read the full story in chapter-04.md.

## Trace Tags

```bash
git log trace/add-task --oneline
git log trace/list-tasks --oneline
git log trace/complete-task --oneline
git log trace/persistence --oneline
```

## License

MIT