use axum::{http::StatusCode, response::IntoResponse};


pub type Result<T> = core::result::Result<T, Error>;

#[derive (Debug)]
#[allow(dead_code)]
pub enum Error {
    LoginFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLER_CLIENT_ERROR").into_response()
    }
} 