table! {
    users (user_id) {
        user_id -> Integer,
        user_name -> Varchar,
        passwd -> Varchar,
        create_time -> Timestamp,
        updata_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}
