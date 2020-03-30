/*
    user {
        user_id
        name
        email
        phone
    }

    room {
        room_id
        owner     User
        managers [User]
        users    [User]
        name
        romm_descript
    }

    message {
        message_id
        from
        room
        message_type  (0 room, 1 p2p, 3 broadcast)
        create_time
    }

*/
