use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
pub mod module;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mine = web::Data::new(module::World {
        resources: Mutex::new(3),
    });

    HttpServer::new(move || App::new().app_data(mine.clone()).service(module::do_work))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
