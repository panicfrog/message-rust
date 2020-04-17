use actix_web::HttpResponse;
use serde::{Serialize};
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum UserResponseState {
    Success = 0,
    Failed = 1,
}

#[derive(Serialize, Debug)]
pub struct UserSuccessResponse<T: Serialize> {
    pub message: String,
    pub state: UserResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub trait UserResponse {
    fn response(&self) -> HttpResponse;
}

impl<T> UserResponse for T
where
    T: Serialize,
{
    fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

pub fn success_with_data<T>(message: &str, data: T) -> HttpResponse
where
    T: Serialize,
{
    UserSuccessResponse::<T> {
        message: String::from(message),
        state: UserResponseState::Success,
        data: Some(data),
    }
    .response()
}

pub fn success_nodata(message: &str) -> HttpResponse {
    let result: UserSuccessResponse<String> = UserSuccessResponse {
        message: String::from(message),
        state: UserResponseState::Success,
        data: None,
    };
    result.response()
}

#[allow(dead_code)]
pub fn fail(message: &str) -> HttpResponse {
    let result: UserSuccessResponse<String> = UserSuccessResponse {
        message: String::from(message),
        state: UserResponseState::Failed,
        data: None,
    };
    result.response()
}

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
