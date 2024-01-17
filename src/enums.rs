use std::cmp::Ordering;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TaskPriority {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh
}

impl TaskPriority {

    pub fn from_string(priority: &str) -> Self
    {
        return match priority {
            "VeryLow" => TaskPriority::VeryLow,
            "Low" => TaskPriority::Low,
            "Medium" => TaskPriority::Medium,
            "High" => TaskPriority::High,
            "VeryHigh" => TaskPriority::VeryHigh,
            _ => panic!("Invalid priority, options are: [VeryLow, Low, Medium, High, VeryHigh]")
        };
    }

    pub fn to_string(&self) -> String {
        match self {
            TaskPriority::VeryLow => "VeryLow".to_string(),
            TaskPriority::Low => "Low".to_string(),
            TaskPriority::Medium => "Medium".to_string(),
            TaskPriority::High => "High".to_string(),
            TaskPriority::VeryHigh => "VeryHigh".to_string(),
        }
    }

    pub fn cmp(&self, other: &TaskPriority) -> Ordering {
        
        match (self, other) {
            (TaskPriority::VeryHigh, TaskPriority::VeryHigh)
            | (TaskPriority::High, TaskPriority::High)
            | (TaskPriority::Medium, TaskPriority::Medium)
            | (TaskPriority::Low, TaskPriority::Low)
            | (TaskPriority::VeryLow, TaskPriority::VeryLow) => Ordering::Equal,
            (TaskPriority::VeryHigh, _) => Ordering::Greater,
            (_, TaskPriority::VeryHigh) => Ordering::Less,

            (TaskPriority::High, _) => {
                if let TaskPriority::Low | TaskPriority::Medium | TaskPriority::VeryLow = other {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (_, TaskPriority::High) => {
                if let TaskPriority::Low | TaskPriority::Medium | TaskPriority::VeryLow = self {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (TaskPriority::Medium, _) => {
                if let TaskPriority::Low | TaskPriority::VeryLow = other {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (_, TaskPriority::Medium) => {
                if let TaskPriority::Low | TaskPriority::VeryLow = self {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (TaskPriority::Low, _) => {
                if let TaskPriority::VeryLow = other {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (_, TaskPriority::Low) => {
                if let TaskPriority::VeryLow = self {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }

    }


}