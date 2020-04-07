use serde::Deserialize;
use actix_web::{web, HttpResponse, Result};
use diesel::{ r2d2::ConnectionManager, MysqlConnection };

use crate::db::user::{ add, verification };
use super::models::success_nodata;

type MysqlDbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Deserialize)]
pub struct User {
    user_name: String,
    passwd: String
}

// 注册
pub async fn register(pool: web::Data<MysqlDbPool>, user: web::Json<User>) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    web::block(move || add(user.user_name.clone(), user.passwd.clone(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            // TODO 错误处理
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(success_nodata("注册成功"))
}

// 登录
pub async fn login(pool: web::Data<MysqlDbPool>, user: web::Json<User>) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    web::block(move || verification(user.user_name.as_str(), user.passwd.as_str(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            // TODO 错误处理
            HttpResponse::InternalServerError().finish()
        })?;

    // TODO 登录成功需要返回token
    Ok(success_nodata("登录成功"))
}
