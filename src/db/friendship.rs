use super::error::{deal_insert_result, deal_query_result, Error};
use super::schema::friendship;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[allow(dead_code)]
#[derive(Queryable)]
pub struct QueryFriendship {
    id: i32,
    pub user_id: i32,
    pub friend_id: i32,
    create_time: NaiveDateTime,
    updata_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "friendship"]
struct InsertableFriends {
    user_id: i32,
    friend_id: i32,
}

#[allow(dead_code)]
fn get_friends_by_id(name: i32, conn: &MysqlConnection) -> Result<Vec<QueryFriendship>, Error>{
    use super::schema::friendship::dsl::*;
    let r = friendship
        .filter(user_id.eq(name))
        .load::<QueryFriendship>(conn);
    deal_query_result(r)
}

#[allow(dead_code)]
fn get_friends_by_friend_id(friend: i32, conn: &MysqlConnection) -> Result<Vec<QueryFriendship>, Error> {
    use super::schema::friendship::dsl::*;
    let r = friendship
        .filter(friend_id.eq(friend))
        .load::<QueryFriendship>(conn);
    deal_query_result(r)
}

#[allow(dead_code)]
fn get_friend_by_name(name: i32, friend_name: i32, conn: &MysqlConnection) -> Result<QueryFriendship, Error> {
    use super::schema::friendship::dsl::*;
    let r: QueryResult<QueryFriendship> = friendship
        .filter(friend_id.eq(friend_name))
        .filter(user_id.eq(name))
        .first(conn);
    deal_query_result(r)
}

#[allow(dead_code)]
fn add_friend_by_name(name: i32, friend_name: i32, conn: &MysqlConnection) -> Result<(), Error> {
    use super::schema::friendship::dsl::*;
    let fs1 = InsertableFriends {
        user_id: name.clone(),
        friend_id: friend_name.clone(),
    };
    let fs2 = InsertableFriends {
        user_id: friend_name.clone(),
        friend_id: name.clone(),
    };
    let r = conn.transaction(|| {
       let r1 = diesel::insert_into(friendship).values(&fs1).execute(conn);
        match r1 {
            Ok(s) => {
                if s != 1 {
                    Ok(s)
                } else {
                    let r2 = diesel::insert_into(friendship).values(&fs2).execute(conn);
                    match r2 {
                        Ok(s) => Ok(s),
                        Err(e) => Err(e)
                    }
                }
            },
            Err(e) => Err(e)
        }
    });
    deal_insert_result(r)
}


