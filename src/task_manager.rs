
use crate::{task::Task, priority_queue::PriorityQueue};
use std::thread;
use std::time::Duration;


pub struct TaskManager {
    tasks_queue: PriorityQueue,
    running_task: Option<Task>,
}

impl TaskManager {


    pub fn new() -> Self {
        TaskManager {
            tasks_queue: PriorityQueue::new(),
            running_task: None
        }
    }

    pub fn create_task(&mut self, user: &str, name: &str, description: &str, priority: &str) {
        let new_task = Task::new(user, name, description, priority);
        self.tasks_queue.enqueue(new_task);
    }   

    pub fn process_tasks(&mut self) {
        while let Some(task) = self.tasks_queue.get() {
            self.running_task = Some(task.clone()); // Assuming Task implements Clone    
            // Call the get_details method on the task to get task details
            let details = task.get_details();
            println!("Current Task details: {:?}", details);
            let duration = Duration::from_secs(5);
            thread::sleep(duration);
        }
    } 

}