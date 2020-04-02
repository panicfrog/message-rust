use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, http, web, HttpResponse, Result};

pub fn write_400<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let s = if let Some(e) = res.response().error() {
        match http::HeaderValue::from_str(e.to_string().as_str()) {
            Ok(v) => v,
            Err(_) => http::HeaderValue::from_static("Error"),
        }
    } else {
        http::HeaderValue::from_static("Error")
    };
    res.response_mut()
        .headers_mut()
        .insert(http::header::WARNING, s);
    Ok(ErrorHandlerResponse::Response(res))
}
