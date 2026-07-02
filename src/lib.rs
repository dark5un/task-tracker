/// A simple task tracker.
///
/// Manages tasks with descriptions, completion status, and persistence.

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
    use mini_clap::{Command, Arg};

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
}