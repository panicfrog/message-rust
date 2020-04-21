use super::response::{ApiResponse, ApiResponseState};
use actix_web::dev::HttpResponseBuilder;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use failure::Fail;

#[allow(dead_code)]
#[derive(Fail, Debug)]
pub enum ApiError {
    #[fail(display = "internal error")]
    InternalError,
    #[fail(display = "bad request")]
    BadRequest,
    #[fail(display = "timeout")]
    Timeout,
    #[fail(display = "user error")]
    UserError(String),
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        if let ApiError::UserError(msg) = self {
            let res: ApiResponse<String> = ApiResponse {
                message: (*msg).clone(),
                state: ApiResponseState::Failed,
                data: None,
            };
            let str = serde_json::to_string(&res).unwrap();
            HttpResponseBuilder::new(self.status_code())
                .set_header(header::CONTENT_TYPE, "application/json")
                .body(str)
        } else {
            HttpResponseBuilder::new(self.status_code())
                .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(self.to_string())
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            ApiError::UserError(_) => StatusCode::OK,
        }
    }
}
