use super::error::ApiError;
use crate::db::error::Error as DBError;
use actix_threadpool::BlockingError;
use actix_web::{web, HttpResponse, ResponseError, Result};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use r2d2_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};

use super::models::success_nodata;
use crate::db::user::{add, verification};

use crate::api::models::success_with_data;
use crate::cache::token;

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
pub async fn register(
    pool: web::Data<MysqlDbPool>,
    user: web::Json<User>,
) -> Result<HttpResponse, ApiError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let (username, password) = (user.user_name.clone(), user.passwd.clone());
    web::block(move || add(username, password, &conn))
        .await
        .and_then(|_| Ok(success_nodata("注册成功")))
        .map_err(|e| match e {
            BlockingError::Canceled => ApiError::InternalError,
            BlockingError::Error(err) => match err {
                DBError::DuplicateData(e) => ApiError::UserError(e),
                DBError::ForeignKeyViolation(e) => ApiError::UserError(e),
                _ => ApiError::InternalError,
            },
        })
}

// 登录
pub async fn login(
    pool: web::Data<MysqlDbPool>,
    user: web::Json<User>,
    redis_pool: web::Data<RedisPool>,
) -> Result<HttpResponse, ApiError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let (username, password) = (user.user_name.clone(), user.passwd.clone());
    let (_username, _password) = (username.clone(), password.clone());
    let query_res =
        web::block(move || verification(_username.as_str(), _password.as_str(), &conn)).await;
    let te = query_res.map_err(|e| match e {
        BlockingError::Canceled => ApiError::InternalError,
        BlockingError::Error(e) => {
            ApiError::UserError("username or password is invalid".to_owned())
        }
    });
    if te.is_err() {
        return Err(te.err().unwrap());
    }

    let redis_conn = &mut redis_pool
        .get()
        .expect("countn't get redis connection from pool");
    let u = User {
        user_name: username,
        passwd: password,
    };
    token::store_value(u, redis_conn)
        .and_then(|t| Ok(success_with_data("登录成功", t)))
        .map_err(|_| ApiError::InternalError)
}
