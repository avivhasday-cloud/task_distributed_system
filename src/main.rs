
mod task_manager;
mod task;
mod priority_queue;
mod enums;
mod worker;

use std::sync::{Arc, Mutex};

use task::Task;
use task_manager::TaskManager;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let task_manager = Arc::new(Mutex::new(TaskManager::new(3)));
    {
        let mut tm = task_manager.lock().unwrap();
    
        tm.start_manager_thread()
    }

    let server_address = "127.0.0.1:8000";
    println!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(task_manager.clone()))
            .route("/tasks/", web::post().to(add_task_to_queue))
            .route("/tasks/running", web::get().to(get_running_tasks))
    })
    .bind(server_address)?
    .run()
    .await
}
