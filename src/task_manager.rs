use std::{num::ParseIntError, ops::Index, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread::{self, JoinHandle}};

use crate::{priority_queue::PriorityQueue, task::Task, worker::{Worker, WorkerStatus}};
use std::time::Duration;

pub struct TaskManager {
    tasks_queue: Arc<Mutex<PriorityQueue>>,
    running_tasks: Arc<Mutex<Vec<Task>>>,
    workers: Arc<Mutex<Vec<Worker>>>,
    manager_thread: Option<JoinHandle<()>>,
    keep_thread_alive: Arc<AtomicBool>,
    num_of_max_workers: usize,
}

impl TaskManager {


    pub fn new(num_of_worker_threads: usize) -> Self {
        let task_manager = TaskManager {
            num_of_max_workers: num_of_worker_threads,
            tasks_queue: Arc::new(Mutex::new(PriorityQueue::new())),
            running_tasks: Arc::new(Mutex::new(Vec::new())),
            manager_thread: None,
            keep_thread_alive: Arc::new(AtomicBool::new(true)),
            workers: Arc::new(Mutex::new(Vec::new())),
        };
        
        task_manager
    }

    pub fn start_manager_thread(&mut self){
        let tasks_queue_clone = self.tasks_queue.clone();
        let running_tasks_clone = self.running_tasks.clone();
        let workers_clone = self.workers.clone();
        let num_of_max_workers = self.num_of_max_workers;
        let keep_alive_clone = self.keep_thread_alive.clone();
        self.manager_thread = Some(thread::spawn(move || {
            TaskManager::handle_dispatching_to_workers(tasks_queue_clone, running_tasks_clone, workers_clone, num_of_max_workers, keep_alive_clone)
        }));
    }


    fn handle_dispatching_to_workers(tasks_queue: Arc<Mutex<PriorityQueue>>, running_tasks: Arc<Mutex<Vec<Task>>>, workers_clone: Arc<Mutex<Vec<Worker>>>, num_of_max_workers: usize, keep_thread_alive: Arc<AtomicBool>) {
        while keep_thread_alive.load(Ordering::SeqCst) {
            let task_option = {
                let mut queue = tasks_queue.lock().unwrap();
                queue.get() // Assuming PriorityQueue has a get method
            };

            if let Some(task) = task_option {
                let mut workers = workers_clone.lock().unwrap();

                if let Some(worker) = workers.iter_mut().find(|worker| matches!(worker.get_status(), WorkerStatus::Ready | WorkerStatus::Idle)) {
                    println!("Assigning task to available worker {}", worker.get_id());
                    worker.set_status(WorkerStatus::Running);
                } else if workers.len() < num_of_max_workers {
                    let running_tasks_worker_clone = running_tasks.clone();
                    let mut new_worker = Worker::new(move || {
                        TaskManager::process_task(task, running_tasks_worker_clone);
                    });
                    println!("Creating a new worker for the task");
                    new_worker.set_status(WorkerStatus::Running);
                    workers.push(new_worker);
            } else {
                    println!("Waiting for an available worker...");
                }

            }


        }
    }

    pub fn create_task(&mut self, new_task: Task) {
        let mut queue = self.tasks_queue.lock().unwrap();
        queue.enqueue(new_task);
    }



    pub fn process_task(task: Task, running_tasks: Arc<Mutex<Vec<Task>>>) {
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
    }
    
    pub fn get_running_tasks(&mut self) -> Vec<Task> {
        let running_tasks = self.running_tasks.lock().unwrap();
        return running_tasks.clone();
    }



}


impl Drop for TaskManager {

    fn drop(&mut self) {
        self.keep_thread_alive.store(false, Ordering::SeqCst);
        if let Some(handle) = self.manager_thread.take() {
            let _ = handle.join();
            println!("Joining mangager thread");
        }
        let mut workers_vector = self.workers.lock().unwrap();
        for worker in workers_vector.iter_mut()  {
            worker.join_handle_thread();
        }
    } 
}