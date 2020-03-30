use super::error;
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

pub fn add(u_name: String, pd: String) -> Result<(), error::Error> {
    use super::schema::users::dsl::*;
    let connection = establish_connection();
    let new_user = InsertableUser {
        user_name: u_name,
        passwd: pd,
    };
    let r = diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection);
    match r {
        Ok(s) => {
            if s == 1 {
                Ok(())
            } else {
                Err(error::Error::InsertNumError)
            }
        }
        Err(e) => {
            if let diesel::result::Error::DatabaseError(UniqueViolation, _) = e {
                Err(error::Error::DuplicateData(e.to_string()))
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn verification(u_name: &str, pd: &str) -> Result<(), error::Error> {
    use super::schema::users::dsl::*;
    let connection = establish_connection();
    let r: QueryResult<QueryUser> = users
        .filter(user_name.eq(u_name))
        .filter(passwd.eq(pd))
        .first(&connection);
    match r {
        Ok(u) => Ok(()),
        Err(e) => {
            if let diesel::NotFound = e {
                Err(error::Error::NotFound)
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}

fn find_with_username(u_name: &str) -> Result<QueryUser, error::Error> {
    use super::schema::users::dsl::*;
    let connection = establish_connection();
    let u: QueryResult<QueryUser> = users.filter(user_name.eq(u_name)).first(&connection);
    match u {
        Ok(u) => Ok(u),
        Err(e) => {
            if let diesel::NotFound = e {
                Err(error::Error::NotFound)
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}

fn find_with_id(u_id: i32) -> Result<QueryUser, error::Error> {
    use super::schema::users::dsl::*;

    let connection = establish_connection();
    let u: QueryResult<QueryUser> = users.filter(user_id.eq(u_id)).first(&connection);
    match u {
        Ok(u) => Ok(u),
        Err(e) => {
            if let diesel::NotFound = e {
                Err(error::Error::NotFound)
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn change_passwd(u_name: String, pd: String) -> Result<(), error::Error> {
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
                match r {
                    Ok(s) => {
                        if s == 1 {
                            Ok(())
                        } else {
                            Err(error::Error::NotFound)
                        }
                    }
                    Err(e) => {
                        if let diesel::NotFound = e {
                            Err(error::Error::NotFound)
                        } else {
                            Err(error::Error::WapperError(e.to_string()))
                        }
                    }
                }
            }
        }
        Err(e) => Err(e),
    }
}
