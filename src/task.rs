
use std::{collections::HashMap, cmp::Ordering};

use crate::enums::TaskPriority;

#[derive(Debug, PartialEq, Eq, Clone)]
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