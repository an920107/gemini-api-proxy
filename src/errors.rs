use actix_web::{http::StatusCode, ResponseError};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct StreamError {
    pub status: StatusCode,
    pub message: String,
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl Error for StreamError {}

impl ResponseError for StreamError {
    fn status_code(&self) -> StatusCode {
        self.status
    }
}
