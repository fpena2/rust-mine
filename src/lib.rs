use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct World {
    pub resources: Mutex<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct Worker {
    pub uid: String,
    pub work: u64,
    // Credit for work that was not accepted
    pub worker_credit: u64,
}

#[derive(Deserialize, Serialize)]
pub struct WorkResponse {
    pub mine_resources: u64,
    pub work_credit: u64,
}

pub async fn do_work(info: web::Json<Worker>, data: web::Data<World>) -> impl Responder {
    let mut shared_data = data.resources.lock().unwrap();
    let mut credit = 0;

    println!("Hello: {}!", info.uid);

    if let Some(result) = shared_data.checked_sub(info.work) {
        println!("Your work has been processed: {}!", info.work);
        *shared_data = result;
    } else {
        println!("Your work was partially processed!");
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
