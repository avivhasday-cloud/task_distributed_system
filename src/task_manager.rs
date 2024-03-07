use std::{num::ParseIntError, ops::Index, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, TryRecvError}, Arc, Mutex}, thread::{self, JoinHandle}};

use uuid::Uuid;

use crate::{priority_queue::PriorityQueue, task::Task, worker::{Worker, WorkerStatus}};
use std::time::Duration;

pub struct TaskManager {
    tasks_queue: Arc<Mutex<PriorityQueue>>,
    running_tasks: Arc<Mutex<Vec<Task>>>,
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
        };
        
        task_manager
    }

    pub fn start_manager_thread(&mut self){
        let tasks_queue_clone = self.tasks_queue.clone();
        let running_tasks_clone = self.running_tasks.clone();
        let num_of_max_workers = self.num_of_max_workers;
        let keep_alive_clone = self.keep_thread_alive.clone();
        self.manager_thread = Some(thread::spawn(move || {
            TaskManager::handle_dispatching_to_workers(tasks_queue_clone, running_tasks_clone, num_of_max_workers, keep_alive_clone)
        }));
    }


    fn handle_dispatching_to_workers(tasks_queue: Arc<Mutex<PriorityQueue>>, running_tasks: Arc<Mutex<Vec<Task>>>,  num_of_max_workers: usize, keep_thread_alive: Arc<AtomicBool>) {
        let mut workers: Vec<Worker> = Vec::new();
        let (tx, rx): (mpsc::Sender<Uuid>, mpsc::Receiver<Uuid>) = mpsc::channel();
        
        while keep_thread_alive.load(Ordering::SeqCst) {

            match rx.try_recv() {
                Ok(worker_id) => {
                    // Find the worker that sent the signal and mark it as Idle
                    if let Some(worker) = workers.iter_mut().find(|w| w.get_id() == worker_id) {
                        println!("Worker {} has completed its task", worker_id);
                        worker.set_status(WorkerStatus::Idle);
                        println!("Worker {} status is now idle", worker_id);

                    }
                },
                Err(TryRecvError::Empty) => {
                    // No worker has finished yet; proceed to check for new tasks
                },
                Err(TryRecvError::Disconnected) => {
                    // Channel has been disconnected; possibly handle error or break
                    break;
                }
            }

            let task_option = {
                let mut queue = tasks_queue.lock().unwrap();
                queue.get()
            };

            if let Some(task) = task_option {

                if let Some(worker) = workers.iter_mut().find(|worker| matches!(worker.get_status(), WorkerStatus::Ready | WorkerStatus::Idle)) {
                    println!("Assigning task with priority {} to available worker {}", task.get_priority_as_str() ,worker.get_id());
                    let worker_id = worker.get_id();
                    let tx_clone = tx.clone();
                    let running_tasks_worker_clone = running_tasks.clone();
                    worker.run_task(move || {
                        TaskManager::process_task(task, running_tasks_worker_clone);
                        tx_clone.send(worker_id).unwrap();
                        
                    });  
                    worker.set_status(WorkerStatus::Running);
                } else if workers.len() < num_of_max_workers {
                    let running_tasks_worker_clone = running_tasks.clone();
                    let mut new_worker = Worker::new();
                    let tx_clone = tx.clone();
                    let worker_id = new_worker.get_id();
                    new_worker.run_task(move || {
                        TaskManager::process_task(task, running_tasks_worker_clone);
                        tx_clone.send(worker_id).unwrap();
                        
                    });     
                    println!("Creating a new worker for the task");
                    new_worker.set_status(WorkerStatus::Running);
                    workers.push(new_worker);
                } else {
                        let mut queue = tasks_queue.lock().unwrap();
                        queue.enqueue(task);
                        println!("Waiting for an available worker...");
                        thread::sleep(Duration::from_secs(10));
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
            println!("Joining manager thread");
        }
    } 
}