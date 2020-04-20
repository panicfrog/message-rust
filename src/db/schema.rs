table! {
    friendship (id) {
        id -> Integer,
        user_id -> Integer,
        friend_id -> Integer,
        create_time -> Timestamp,
        updata_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    message (id) {
        id -> Integer,
        from -> Integer,
        to -> Integer,
        content -> Varchar,
        #[sql_name = "type"]
        type_ -> Integer,
        create_time -> Timestamp,
        updata_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    room (id) {
        id -> Integer,
        name -> Varchar,
        description -> Varchar,
        create_time -> Timestamp,
        updata_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    room_members (id) {
        id -> Integer,
        room_id -> Integer,
        user_id -> Integer,
        role -> Integer,
        create_time -> Timestamp,
        updata_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

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

joinable!(message -> users (from));
joinable!(room_members -> room (room_id));
joinable!(room_members -> users (user_id));

allow_tables_to_appear_in_same_query!(
    friendship,
    message,
    room,
    room_members,
    users,
);
