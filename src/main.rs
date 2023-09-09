use actix_web::{web, App, HttpServer};
use rust_mine::{do_work, World};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mine = web::Data::new(World {
        resources: Mutex::new(3),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(mine.clone())
            .service(web::resource("/do_work").route(web::post().to(do_work)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
