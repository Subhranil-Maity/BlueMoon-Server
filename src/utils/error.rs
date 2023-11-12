use axum::{http::StatusCode, response::IntoResponse};

use crate::log::err;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    FileOrFolderNotFound,
    AccessDeniedBySystem,
    NotAValidPath,
    QueryParamMissingOrNotValid,
    UNKNOWN,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        err(format!("{self:?}"));

        (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled_client_error").into_response()
    }
}
