use actix_web::{HttpResponse, error::ResponseError};
use log::error;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(String),
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::DatabaseError(msg) => {
                // Log the actual error internally, but don't expose it
                error!("Database error: {}", msg);
                HttpResponse::InternalServerError().json("Internal server error")
            }
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            ApiError::InternalServerError(msg) => {
                // Log the actual error internally
                error!("Internal server error: {}", msg);
                HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(err: sea_orm::DbErr) -> Self {
        // Log the actual database error but don't expose it
        error!("Database error occurred: {:?}", err);
        ApiError::DatabaseError(err.to_string())
    }
}

// You can also add more conversions for common errors
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        error!("Unexpected error: {:?}", err);
        ApiError::InternalServerError(err.to_string())
    }
}
