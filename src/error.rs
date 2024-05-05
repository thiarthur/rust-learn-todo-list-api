use derive_more::{Display, Error}; // Ensure derive_more is imported
use ntex::web;
use ntex::web::HttpResponse;
use serde::Serialize;
use std::fmt::{Debug, Display};
use web::error::{InternalError, WebResponseError};

pub trait ApplicationErrorTrait {
    fn to_http_response(&self) -> HttpResponse;
}

#[derive(Debug, Display, Error, Serialize, Clone)]
#[display(fmt = "Error: {}", message)]
pub struct NotFoundError {
    pub message: String,
}

impl ApplicationErrorTrait for NotFoundError {
    fn to_http_response(&self) -> HttpResponse {
        HttpResponse::NotFound().json(self) // Return 404 with the error message
    }
}

impl WebResponseError for NotFoundError {}

#[derive(Debug, Display, Error, Serialize, Clone)]
#[display(fmt = "Error: {}", message)]
pub struct SerializationError {
    pub message: String,
}

impl ApplicationErrorTrait for SerializationError {
    fn to_http_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(self)
    }
}

impl WebResponseError for SerializationError {}

pub fn create_internal_error<T>(e: T) -> web::Error
where
    T: ApplicationErrorTrait + Debug + Display + 'static + Clone,
{
    InternalError::from_response(e.clone(), e.to_http_response()).into()
}
