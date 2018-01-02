use std::fmt;
use std::io::Cursor;
use std::convert::From;
use std::error::Error as StdError;

use rocket::request::Request;
use rocket::http::{Status, ContentType};
use rocket::response::{Response,Responder};
use diesel::result::Error as DieselError;
use diesel::result::DatabaseErrorKind;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ApiError {
    RecordNotFound,
    UniqueViolation,
    NotNullViolation,
    InvalidRequestError,
    InvalidUuidError,
    InvalidJsonError,
    EmptyNameError,
    InternalServerError,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::RecordNotFound      => f.write_str("RecordNotFound"),
            ApiError::UniqueViolation     => f.write_str("UniqueViolation"),
            ApiError::NotNullViolation    => f.write_str("NotNullViolation"),
            ApiError::InvalidUuidError    => f.write_str("InvalidUuidError"),
            ApiError::InvalidJsonError    => f.write_str("InvalidJsonError"),
            ApiError::EmptyNameError      => f.write_str("InvalidNameError"),
            ApiError::InvalidRequestError => f.write_str("InvalidRequestError"),
            ApiError::InternalServerError => f.write_str("InternalServerError"),
        }
    }
}

impl StdError for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::RecordNotFound      => "Record not found",
            ApiError::UniqueViolation     => "Unique value violation",
            ApiError::NotNullViolation    => "Value not null violation",
            ApiError::InvalidJsonError    => "Json was not well formed",
            ApiError::InvalidUuidError    => "Uuid was not well formed",
            ApiError::EmptyNameError      => "Name was empty",
            ApiError::InvalidRequestError => "Request was not well formed",
            ApiError::InternalServerError => "Unspecified internal server error",
        }
    }
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &Request ) -> Result<Response<'r>, Status> {
        let (msg,stat) = match self {
            ApiError::RecordNotFound      => {("Record not found", Status::NotFound)},
            ApiError::UniqueViolation     => {("Unique key violation", Status::BadRequest)},
            ApiError::NotNullViolation    => {("Column not null violation",Status::BadRequest)},
            ApiError::InvalidRequestError => {("Request was not well formed",Status::BadRequest)},
            ApiError::InvalidUuidError    => {("Uuid was not well formed",Status::BadRequest)},
            ApiError::InvalidJsonError    => {("Json was not well formed",Status::BadRequest)},
            ApiError::EmptyNameError      => {("Name was not empty",Status::BadRequest)},
            ApiError::InternalServerError => {("Internal server error",Status::InternalServerError)},
        };
        
        let resp = format!("{{'error':'{}'}}",msg);
        Response::build()
            .status(stat)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(resp))
            .ok()
        
 
    }
}

impl From<DieselError> for ApiError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => ApiError::RecordNotFound,
            _                     => ApiError::InternalServerError,
        }
    }
}

impl From<DatabaseErrorKind> for ApiError {
    fn from(e:DatabaseErrorKind) -> Self {
        match e {
            DatabaseErrorKind::UniqueViolation => ApiError::UniqueViolation,
            _                                  => ApiError::InternalServerError,
        }
    }
}
