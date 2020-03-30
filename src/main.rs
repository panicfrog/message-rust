#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix::*;
use actix_web::{web, App, HttpServer};
use chat::{route, server};

mod api;
mod chat;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let srv = server::ChatServer::default().start();
    HttpServer::new(move || {
        App::new()
            .data(srv.clone())
            .service(web::resource("/ws").to(route::chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
