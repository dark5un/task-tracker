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

    // Cycle 1: Create a task with a description
    #[test]
    fn test_create_task_with_description() {
        let task = Task::new("Buy milk");
        assert_eq!(task.description, "Buy milk");
        assert!(!task.done);
    }
}