#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix::*;
use actix_web::{http, middleware, middleware::errhandlers::ErrorHandlers, web, App, HttpServer};
use api::route::write_400;
use chat::{route, server};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use std::env;

mod api;
mod chat;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv::dotenv().ok();
    let connspec = env::var("DATABASE_URL").expect("need set `DATABASE_URL` at .env");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Fail to create pool.");
    let srv = server::ChatServer::default().start();
    HttpServer::new(move || {
        App::new()
            .data(srv.clone())
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(ErrorHandlers::new().handler(http::StatusCode::BAD_REQUEST, write_400))
            .service(web::resource("/ws").to(route::chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
