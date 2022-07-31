use actix_web::{http::header::ContentType, http::StatusCode, HttpResponse, ResponseError};
use strum_macros::Display;
use utoipa::Component;

#[derive(Debug, Display, Component)]
pub enum TransactionError {
    TransactionNotFound,
    TransactionInvalid,
    DatabaseError(sqlx::Error),

}

impl From<sqlx::Error> for TransactionError {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseError(e)
    }
}

impl ResponseError for TransactionError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::TransactionNotFound => StatusCode::NOT_FOUND,
            Self::TransactionInvalid => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self.to_string())
    }
}
