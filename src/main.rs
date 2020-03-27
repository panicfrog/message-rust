use actix::*;
use actix_web::{web, App, HttpServer};

mod chat_route;
mod chat_server;
mod chat_model;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let server = chat_server::ChatServer::default().start();
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(web::resource("/ws").to(chat_route::chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
