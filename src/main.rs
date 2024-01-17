mod task_manager;
mod task;
mod priority_queue;
mod enums;
use crate::task_manager::TaskManager;


fn main() {
    println!("Hello, world!");

    let mut task_manager =  TaskManager::new();
    task_manager.create_task("Aviv", "First Task", "Task description", "Medium");
    task_manager.create_task("Rotem", "Second Task", "Task description2", "High");
    task_manager.create_task("Aviv", "Forth Task", "Task description", "Medium");
    task_manager.create_task("Shit", "Third Task", "Task description2", "VeryLow");

    task_manager.process_tasks()
}
