use std::thread::{self, JoinHandle};

extern crate uuid;

use uuid::Uuid;
#[derive(Debug,Clone, Copy)]
pub enum WorkerStatus {
    Ready,
    Busy,
    Running,
    Idle
}

impl WorkerStatus {
    
    pub fn from_string(status: &str) -> Self
    {
        return match status {
            "Ready" => WorkerStatus::Ready,
            "Busy" => WorkerStatus::Busy,
            "Running" => WorkerStatus::Running,
            "Idle" => WorkerStatus::Idle,
            _ => panic!("Invalid work status! Options are [Ready, Busy, Running, Idle]")
        }
    }

}

pub struct Worker {
    id: Uuid,
    thread_handle: Option<JoinHandle<()>>,
    idle_time: u32,
    status: WorkerStatus

}

impl Worker {

    pub fn new() -> Self 
    {
        Worker {
            id: Uuid::new_v4(),
            thread_handle: None,
            idle_time: 0,
            status: WorkerStatus::from_string("Busy")
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn get_status(&self) -> WorkerStatus {
        self.status.clone()
    }

    pub fn set_status(&mut self, new_status: WorkerStatus) {
        self.status = new_status;
    }

    pub fn get_idle_time(&self) -> u32 {
        self.idle_time
    }

    pub fn set_idle_time(&mut self, new_idle_time: u32) {
        self.idle_time = new_idle_time;
    }

    pub fn run_task<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static, 
    {
        self.thread_handle = Some(thread::spawn(f))
    }

    pub fn join_handle_thread(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
            println!("Join worker thread: {}", self.get_id());
        }
    }

}

impl Drop for Worker {

    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
            println!("Joining worker {} thread", self.get_id());
        }
    } 
}
