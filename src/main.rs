use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

struct World {
    resources: Mutex<u64>,
}

#[derive(Deserialize)]
struct Worker {
    uid: String,
    work: u64,
}

#[post("/do_work")]
async fn do_work(info: web::Json<Worker>, data: web::Data<World>) -> impl Responder {
    let mut shared_data = data.resources.lock().unwrap();

    println!("Welcome {}!", info.uid);

    if let Some(result) = shared_data.checked_sub(info.work) {
        println!("Your work has been processed: {}!", info.work);
        *shared_data = result;
    } else {
        println!("You work could not be processed!");
    }

    println!("Current mine resources: {}", shared_data);
    HttpResponse::Ok().body(shared_data.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mine = web::Data::new(World {
        resources: Mutex::new(10),
    });

    HttpServer::new(move || App::new().app_data(mine.clone()).service(do_work))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
