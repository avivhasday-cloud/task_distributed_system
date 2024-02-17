use std::{num::ParseIntError, ops::Index, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

use crate::{task::Task, priority_queue::PriorityQueue};
use std::time::Duration;

pub struct TaskManager {
    tasks_queue: Arc<Mutex<PriorityQueue>>,
    running_tasks: Arc<Mutex<Vec<Task>>>,
    thread_handle: Option<JoinHandle<()>>
}

impl TaskManager {


    pub fn new() -> Self {
        TaskManager {
            tasks_queue: Arc::new(Mutex::new(PriorityQueue::new())),
            running_tasks: Arc::new(Mutex::new(Vec::new())),
            thread_handle: None,
        }
    }

    pub fn create_task(&mut self, new_task: Task) {
        let mut queue = self.tasks_queue.lock().unwrap();
        queue.enqueue(new_task);
        let tasks_queue_clone = self.tasks_queue.clone();
        let running_tasks_clone = self.running_tasks.clone();

        // Correctly wrapping the JoinHandle<()> in Some()
        self.thread_handle = Some(thread::spawn(move || {
            // Simulate processing tasks
            TaskManager::process_tasks(tasks_queue_clone, running_tasks_clone);
        }));
    }

    pub fn process_tasks(tasks_queue: Arc<Mutex<PriorityQueue>>, running_tasks: Arc<Mutex<Vec<Task>>>) {
        loop {
            let task_option = {
                let mut queue = tasks_queue.lock().unwrap();
                queue.get() // Assuming PriorityQueue has a get method
            };
    
            if let Some(task) = task_option {
                {
                    let mut running_tasks = running_tasks.lock().unwrap();
                    running_tasks.push(task.clone());
                }
                
                println!("Processing task");
                // Simulate task processing with a sleep
                thread::sleep(Duration::from_secs(30));
    
                {
                    let mut running_tasks = running_tasks.lock().unwrap();
                    if let Some(index) = running_tasks.iter().position(|t| t.get_name() == task.get_name()) {
                        running_tasks.remove(index);
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn get_running_tasks(&mut self) -> Vec<Task> {
        let running_tasks = self.running_tasks.lock().unwrap();
        return running_tasks.clone();
    }

    pub fn grettings(& self) {
        println!("Hi from task manager");
    }

}

impl Drop for TaskManager {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
            println!("Thread joined successfully.");
        }
    }
}