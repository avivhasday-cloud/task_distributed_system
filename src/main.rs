
mod task_manager;
mod task;
mod priority_queue;
mod enums;

use std::sync::{Arc, Mutex};

use task::Task;
use task_manager::TaskManager;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet(task_manager: web::Data<Arc<Mutex<TaskManager>>>) -> impl Responder {
    let tm = task_manager.lock().unwrap();
    tm.grettings();
    HttpResponse::Ok()
}

async fn add_task_to_queue(task_manager: web::Data<Arc<Mutex<TaskManager>>>, task: web::Json<Task>) -> impl Responder {
    let mut tm = task_manager.lock().unwrap();
    tm.create_task(task.clone());
    HttpResponse::Ok()
}

async fn get_running_tasks(task_manager: web::Data<Arc<Mutex<TaskManager>>>) -> impl Responder {
    let mut tm = task_manager.lock().unwrap();
    let running_tasks = tm.get_running_tasks();
    HttpResponse::Ok().json(running_tasks)
}

#[actix_web::main] // Marks the async main function as the entry point of the application
async fn main() -> std::io::Result<()> {

    let task_manager = Arc::new(Mutex::new(TaskManager::new()));

    let server_address = "127.0.0.1:8000";
    println!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(task_manager.clone()))
            .route("/", web::get().to(greet))
            .route("/tasks/", web::post().to(add_task_to_queue))
            .route("/tasks/running", web::get().to(get_running_tasks))
    })
    .bind(server_address)?
    .run()
    .await
}

// fn main() {

//     let mut task_manager =  TaskManager::new();
//     task_manager.create_task("Aviv", "First Task", "Task description", "Medium");
//     task_manager.create_task("Rotem", "Second Task", "Task description2", "High");
//     task_manager.create_task("Aviv", "Forth Task", "Task description", "Medium");
//     task_manager.create_task("Shit", "Third Task", "Task description2", "VeryLow");

//     task_manager.process_tasks()
// }
