
use std::{collections::HashMap, cmp::Ordering};
use serde::{Deserialize, Serialize};

use crate::enums::TaskPriority;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Task {
    user: String,
    name: String,
    description: String,
    priority: TaskPriority,

}

impl Task {
    
    pub fn new(user: &str, name: &str, description: &str, priority: &str) -> Self {
        Task {
            user: user.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            priority: TaskPriority::from_string(priority),

        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_details(&self) -> HashMap<String, String> {
        let mut task_details = HashMap::new();

        task_details.insert("user".to_string(), self.user.clone());
        task_details.insert("name".to_string(), self.name.clone());
        task_details.insert("description".to_string(), self.description.clone());
        task_details.insert("priority".to_string(), self.priority.to_string());

        return task_details;
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare tasks based on their priority
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        task: Task,
    }
    impl Drop for TestContext {
        fn drop(&mut self) {
            println!("Test teardown ...");
        }
    }

    fn setup() -> TestContext {

        TestContext {
            task: Task::new("test-user","test task", "test description", "High")
        }
    }

    #[test]
    fn test_create_task_succesfully() {
        let task = Task::new("test-user","test task", "test description", "High");
        assert_eq!(task.user, "test-user");
        assert_eq!(task.name, "test task");
        assert_eq!(task.description, "test description");
        assert_eq!(task.priority, TaskPriority::from_string("High"));

    }

    #[test]
    fn test_get_name() {
        let ctx = setup();
        assert_eq!(ctx.task.name, "test task");
    }

    #[test]
    fn test_get_details() {
        // Setup - create a task instance
        let task = Task {
            user: "John Doe".to_string(),
            name: "Task 1".to_string(),
            description: "This is a test task".to_string(),
            priority: TaskPriority::from_string("High"),
        };

        // Exercise - call get_details
        let details = task.get_details();

        // Verify - check that the HashMap contains the correct key-value pairs
        assert_eq!(details.get("user"), Some(&"John Doe".to_string()));
        assert_eq!(details.get("name"), Some(&"Task 1".to_string()));
        assert_eq!(details.get("description"), Some(&"This is a test task".to_string()));
        assert_eq!(details.get("priority"), Some(&TaskPriority::to_string(&TaskPriority::High))); // Adjust if your Priority::to_string() outputs differently

        // Optionally, verify the HashMap contains only the keys we expect
        assert_eq!(details.len(), 4);
    }
}