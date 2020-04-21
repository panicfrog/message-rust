use actix_web::HttpResponse;
use serde::Serialize;
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ApiResponseState {
    Success = 0,
    Failed = 1,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse<T: Serialize> {
    pub message: String,
    pub state: ApiResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub trait UserResp {
    fn response(&self) -> HttpResponse;
}

impl<T> UserResp for T
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
    ApiResponse::<T> {
        message: String::from(message),
        state: ApiResponseState::Success,
        data: Some(data),
    }
    .response()
}

pub fn success_nodata(message: &str) -> HttpResponse {
    let result: ApiResponse<String> = ApiResponse {
        message: String::from(message),
        state: ApiResponseState::Success,
        data: None,
    };
    result.response()
}
