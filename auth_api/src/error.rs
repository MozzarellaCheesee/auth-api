use actix_web::{HttpResponse};
use actix_web::error::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    HashingError(argon2::password_hash::Error),
    #[error("{0}")]
    DbError(#[from] diesel::result::Error),
    #[error("{0}")]
    EmailAlreadyExists(String),
    #[error("{0}")]
    UsernameAlreadyExists(String),
    #[error("{0}")]
    EmailNotConfirmed(String),
    #[error("{0}")]
    UserIsNotExist(String),
    #[error("{0}")]
    WrongPasswordError(String),
    #[error("{0}")]
    TokenCreationError(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    ExpiredTokenError(String),
    #[error("{0}")]
    RevorkTokenError(String),
    #[error("{0}")]
    TokenIsNotValid(String),
    
    
}

impl ResponseError for CustomError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CustomError::HashingError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::DbError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::TokenCreationError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::TokenIsNotValid(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::RevorkTokenError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,

            CustomError::UsernameAlreadyExists(_) => actix_web::http::StatusCode::CONFLICT,

            CustomError::EmailAlreadyExists(_) => actix_web::http::StatusCode::CONFLICT,
            CustomError::EmailNotConfirmed(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            CustomError::UserIsNotExist(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            CustomError::WrongPasswordError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            CustomError::ExpiredTokenError(_) => actix_web::http::StatusCode::UNAUTHORIZED,

            CustomError::InvalidInput(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}

