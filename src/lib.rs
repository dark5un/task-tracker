/// A simple task tracker.
///
/// Manages tasks with descriptions, completion status, and persistence.
use std::fmt;

/// A collection of tasks.
#[derive(Debug, Clone)]
pub struct TaskStore {
    tasks: Vec<Task>,
}

impl TaskStore {
    /// Create an empty task store.
    pub fn new() -> Self {
        TaskStore { tasks: Vec::new() }
    }

    /// Add a task to the store.
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Number of tasks in the store.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// All tasks in insertion order.
    pub fn all(&self) -> &[Task] {
        &self.tasks
    }
}

impl fmt::Display for TaskStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tasks.is_empty() {
            return writeln!(f, "No tasks yet. Add one with `task add <description>`");
        }
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[x]" } else { "[ ]" };
            writeln!(f, "{} {}  {}", i + 1, status, task.description)?;
        }
        Ok(())
    }
}

/// A single task item.
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

impl Task {
    /// Create a new incomplete task.
    pub fn new(description: &str) -> Self {
        Task {
            description: description.to_string(),
            done: false,
        }
    }
}

// ---- Cycle 1: Task creation ----

#[cfg(test)]
mod tests {
    use super::*;
    use mini_clap::{Command, Arg, Flag};

    // Cycle 1: Create a task with a description
    #[test]
    fn test_create_task_with_description() {
        let task = Task::new("Buy milk");
        assert_eq!(task.description, "Buy milk");
        assert!(!task.done);
    }

    // Cycle 2: Parse `task-tracker add "Buy milk"` via mini-clap subcommand
    #[test]
    fn test_add_subcommand_parses_description() {
        let add_cmd = Command::new("add")
            .about("Add a new task")
            .arg(Arg::new("description"));
        let cmd = Command::new("task-tracker")
            .about("A simple task tracker")
            .subcommand(add_cmd);

        let matches = cmd.parse(&["add".to_string(), "Buy milk".to_string()]).unwrap();
        let (name, sub) = matches.subcommand().unwrap();
        assert_eq!(name, "add");
        assert_eq!(sub.get("description"), Some("Buy milk"));
    }

    // Cycle 3: TaskStore — add and list tasks
    #[test]
    fn test_store_add_and_count() {
        let mut store = TaskStore::new();
        assert_eq!(store.len(), 0);
        store.add(Task::new("Buy milk"));
        assert_eq!(store.len(), 1);
    }

    // Cycle 4: list subcommand — parse via mini-clap
    #[test]
    fn test_list_subcommand_parses() {
        let list_cmd = Command::new("list")
            .about("List all tasks");
        let cmd = Command::new("task-tracker")
            .about("A simple task tracker")
            .subcommand(list_cmd);

        let matches = cmd.parse(&["list".to_string()]).unwrap();
        let (name, _sub) = matches.subcommand().unwrap();
        assert_eq!(name, "list");
    }
}