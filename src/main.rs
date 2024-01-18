mod task_manager;
mod task;
mod priority_queue;
mod enums;


use actix_web::{web, App, HttpServer, Responder};

async fn greet() -> impl Responder {
    "Hello, Actix!"
}

#[actix_web::main] // Marks the async main function as the entry point of the application
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8000";
    println!("Starting server at http://{}", server_address);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
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
