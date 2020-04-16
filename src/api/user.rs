use actix_web::{web, HttpResponse, Result};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use r2d2_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};

use super::models::success_nodata;
use crate::db::user::{add, verification};

use crate::api::models::{fail, success_with_data};
use crate::cache::token;
use std::mem;

type MysqlDbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type RedisPool = r2d2_redis::r2d2::Pool<RedisConnectionManager>;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub user_name: String,
    pub passwd: String,
}

impl token::Identifier for User {
    fn id(&self) -> String {
        self.user_name.clone()
    }
}

// 注册
pub async fn register(pool: web::Data<MysqlDbPool>, user: web::Json<User>) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let (username, password) = (user.user_name.clone(), user.passwd.clone());
    web::block(move || add(username, password, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            // TODO 错误处理
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(success_nodata("注册成功"))
}

// 登录
pub async fn login(
    pool: web::Data<MysqlDbPool>,
    user: web::Json<User>,
    redis_pool: web::Data<RedisPool>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let (username, password) = (user.user_name.clone(), user.passwd.clone());
    let (_username, _password) = (username.clone(), password.clone());
    web::block(move || verification(_username.as_str(), _password.as_str(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            // TODO 错误处理
            HttpResponse::InternalServerError().finish()
        })?;
    let redis_conn = &mut redis_pool
        .get()
        .expect("countn't get redis connection from pool");
    let u = User {
        user_name: username,
        passwd: password,
    };
    match token::store_value(u, redis_conn) {
        Ok(t) => Ok(success_with_data("登录成功", t)),
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "reids保存token错误",
        )),
    }
}
