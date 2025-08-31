use std::fmt::Display;

use axum::{
    http::{StatusCode, uri::InvalidUri},
    response::{IntoResponse, Response},
};
use db::{PoolError, TokioPostgresError};

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Database(ref cause) => write!(f, "Database Error: {}", cause),
        }
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::FaultySetup(cause) => (StatusCode::UNPROCESSABLE_ENTITY, cause),
            CustomError::Database(cause) => (StatusCode::UNPROCESSABLE_ENTITY, cause),
        };
        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<InvalidUri> for CustomError {
    fn from(err: InvalidUri) -> Self {
        Self::FaultySetup(err.to_string())
    }
}

impl From<TokioPostgresError> for CustomError {
    fn from(err: TokioPostgresError) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<PoolError> for CustomError {
    fn from(err: PoolError) -> Self {
        Self::Database(err.to_string())
    }
}
