#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2_redis;
extern crate base64;

use actix::*;
use actix_web::{http, middleware, middleware::errhandlers::ErrorHandlers, web, App, HttpServer, cookie};
use api::route::write_400;
use chat::{route, server};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use r2d2_redis::{r2d2 as redis_r2d2, RedisConnectionManager};
use std::env;

mod api;
mod cache;
mod chat;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,actix_redis=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let mysql_connspec = env::var("DATABASE_URL").expect("need set `DATABASE_URL` at .env");
    let mysql_manager = ConnectionManager::<MysqlConnection>::new(mysql_connspec);
    let mysql_pool = r2d2::Pool::builder()
        .build(mysql_manager)
        .expect("Fail to create pool.");

    let redis_connspec = env::var("REDIS_URL").expect("need set `REDIS_URL at .env");
    let redis_manager =
        RedisConnectionManager::new(redis_connspec).expect("Fail to create redis manager");
    let redis_pool = redis_r2d2::Pool::builder()
        .build(redis_manager)
        .expect("Fail to create redis pool");

    let srv = server::ChatServer::default().start();

    // let sess = RedisSession::new(redis_connspec, &[0;32]);
    HttpServer::new(move || {
        App::new()
            .data(srv.clone())
            .data(mysql_pool.clone())
            .data(redis_pool.clone())
            // .wrap(RedisSession::new(redis_connspec.clone(), &[0;32]))
            .wrap(middleware::Logger::default())
            .wrap(ErrorHandlers::new().handler(http::StatusCode::BAD_REQUEST, write_400))
            .service(
                web::scope("/api/v1")
                    .route("/user/register", web::post().to(api::user::register))
                    .route("/user/login", web::post().to(api::user::login)),
            )
            .service(web::resource("/ws").to(route::chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
