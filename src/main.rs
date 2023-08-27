use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct World {
    resources: Mutex<u64>,
}

#[derive(Deserialize)]
struct Worker {
    uid: String,
    work: u64,
    // Credit for work that was not accepted
    worker_credit: u64,
}

#[derive(Serialize)]
struct WorkResponse {
    mine_resources: u64,
    work_credit: u64,
}

#[post("/do_work")]
async fn do_work(info: web::Json<Worker>, data: web::Data<World>) -> impl Responder {
    let mut shared_data = data.resources.lock().unwrap();
    let mut credit = 0;

    if let Some(result) = shared_data.checked_sub(info.work) {
        println!("Your work has been processed: {}!", info.work);
        *shared_data = result;
    } else {
        println!("You work was partially processed!");
        if let Some(c) = info.work.checked_sub(*shared_data) {
            // TODO: this breaks integrity since workers can provide their own credit
            credit = c + info.worker_credit;
            *shared_data = 0;
        } else {
            todo!()
        };
    }

    let response = WorkResponse {
        mine_resources: *shared_data,
        work_credit: credit,
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mine = web::Data::new(World {
        resources: Mutex::new(3),
    });

    HttpServer::new(move || App::new().app_data(mine.clone()).service(do_work))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
