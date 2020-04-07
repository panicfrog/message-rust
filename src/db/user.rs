use super::error::{deal_insert_result, deal_query_result, deal_update_result, Error};
use super::establish_connection;
use super::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;

#[derive(Queryable)]
pub struct QueryUser {
    pub user_id: i32,
    pub user_name: String,
    pub passwd: String,
    create_time: NaiveDateTime,
    updata_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUser {
    user_name: String,
    passwd: String,
}

pub fn add(u_name: String, pd: String, conn: &MysqlConnection) -> Result<(), Error> {
    use super::schema::users::dsl::*;
    // let connection = establish_connection();
    let new_user = InsertableUser {
        user_name: u_name,
        passwd: pd,
    };
    let r = diesel::insert_into(users)
        .values(&new_user)
        .execute(conn);
    deal_insert_result(r)
}

pub fn verification(u_name: &str, pd: &str, conn: &MysqlConnection) -> Result<QueryUser, Error> {
    use super::schema::users::dsl::*;
    // let connection = establish_connection();
    let r: QueryResult<QueryUser> = users
        .filter(user_name.eq(u_name))
        .filter(passwd.eq(pd))
        .first(conn);
    deal_query_result(r)
}

fn find_with_username(u_name: &str) -> Result<QueryUser, Error> {
    use super::schema::users::dsl::*;
    let connection = establish_connection();
    let r: QueryResult<QueryUser> = users.filter(user_name.eq(u_name)).first(&connection);
    deal_query_result(r)
}

fn find_with_id(u_id: i32) -> Result<QueryUser, Error> {
    use super::schema::users::dsl::*;

    let connection = establish_connection();
    let r: QueryResult<QueryUser> = users.filter(user_id.eq(u_id)).first(&connection);
    deal_query_result(r)
}

pub fn change_passwd(u_name: String, pd: String) -> Result<(), Error> {
    use super::schema::users::dsl::*;
    let u = find_with_username(u_name.as_str());
    match u {
        Ok(_u) => {
            // TODO: 是否给提示？
            if pd == _u.passwd {
                Ok(())
            } else {
                let connection = establish_connection();
                let r = diesel::update(users.find(_u.user_id))
                    .set(passwd.eq(pd))
                    .execute(&connection);
                deal_update_result(r)
            }
        }
        Err(e) => Err(e),
    }
}
