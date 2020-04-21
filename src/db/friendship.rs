use super::error::{deal_insert_result, deal_query_result, deal_update_result, Error};
use super::schema::friendship;
use super::user::find_with_username;
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
pub fn get_friends_by_name(
    name: &str,
    conn: &MysqlConnection,
) -> Result<Vec<QueryFriendship>, Error> {
    use super::schema::friendship::dsl::*;
    find_with_username(name, conn).and_then(|u| {
        let fr = friendship
            .filter(user_id.eq(u.user_id))
            .filter(delete_time.is_null())
            .load::<QueryFriendship>(conn);
        deal_query_result(fr)
    })
}

#[allow(dead_code)]
pub fn get_friends_by_friend_name(
    name: &str,
    conn: &MysqlConnection,
) -> Result<Vec<QueryFriendship>, Error> {
    use super::schema::friendship::dsl::*;
    find_with_username(name, conn).and_then(|u| {
        let fr = friendship
            .filter(friend_id.eq(u.user_id))
            .filter(delete_time.is_null())
            .load::<QueryFriendship>(conn);
        deal_query_result(fr)
    })
}

#[allow(dead_code)]
pub fn get_friends(
    name: &str,
    friend_name: &str,
    conn: &MysqlConnection,
) -> Result<QueryFriendship, Error> {
    use super::schema::friendship::dsl::*;
    let u1 = find_with_username(name, conn);
    if u1.is_err() {
        return Err(u1.err().unwrap());
    }
    let u2 = find_with_username(friend_name, conn);
    if u2.is_err() {
        return Err(u2.err().unwrap());
    }

    let fr: QueryResult<QueryFriendship> = friendship
        .filter(user_id.eq(u1.unwrap().user_id))
        .filter(friend_id.eq(u2.unwrap().user_id))
        .filter(delete_time.is_null())
        .first(conn);

    deal_query_result(fr)
}

#[allow(dead_code)]
pub fn and_friend(name: &str, friend_name: &str, conn: &MysqlConnection) -> Result<(), Error> {
    use super::schema::friendship::dsl::*;
    let u1 = find_with_username(name, conn);
    if u1.is_err() {
        return Err(u1.err().unwrap());
    }
    let u2 = find_with_username(friend_name, conn);
    if u2.is_err() {
        return Err(u2.err().unwrap());
    }

    let id1 = u1.unwrap().user_id;
    let id2 = u2.unwrap().user_id;
    let fs1 = InsertableFriends {
        user_id: id1.clone(),
        friend_id: id2.clone(),
    };
    let fs2 = InsertableFriends {
        user_id: id2.clone(),
        friend_id: id1.clone(),
    };

    let r = conn.transaction(|| {
        let r1 = diesel::insert_into(friendship).values(&fs1).execute(conn);
        r1.and_then(|s| {
            if s != 1 {
                Ok(s)
            } else {
                diesel::insert_into(friendship).values(&fs2).execute(conn)
            }
        })
    });
    deal_insert_result(r)
}

#[allow(dead_code)]
fn delete_friend(name: &str, friend_name: &str, conn: &MysqlConnection) -> Result<(), Error> {
    use super::schema::friendship::dsl::*;
    let u1 = find_with_username(name, conn);
    if u1.is_err() {
        return Err(u1.err().unwrap());
    }
    let u2 = find_with_username(friend_name, conn);
    if u2.is_err() {
        return Err(u2.err().unwrap());
    }

    let id1 = u1.unwrap().user_id;
    let id2 = u2.unwrap().user_id;

    let r = conn.transaction(|| {
        let r1 = diesel::update(
            friendship
                .filter(user_id.eq(id1.clone()))
                .filter(friend_id.eq(id2.clone())),
        )
        .set(delete_time.eq(diesel::dsl::now))
        .execute(conn);
        r1.and_then(|s| {
            if s != 1 {
                Ok(s)
            } else {
                diesel::update(
                    friendship
                        .filter(user_id.eq(id1.clone()))
                        .filter(friend_id.eq(id2.clone())),
                )
                .set(delete_time.eq(diesel::dsl::now))
                .execute(conn)
            }
        })
    });
    deal_update_result(r)
}
