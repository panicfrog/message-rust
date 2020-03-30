use actix::*;
use actix_web::{web, App, HttpServer};
use chat::{server, route};

mod chat;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let server = server::ChatServer::default().start();
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(web::resource("/ws").to(route::chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
