use derive_more::{Display, Error}; // Ensure derive_more is imported
use ntex::web;
use ntex::web::HttpResponse;
use serde::Serialize;
use std::fmt::{Debug, Display};
use web::error::{InternalError, WebResponseError};

pub trait InternalErrorTrait {
    fn to_http_response(&self) -> HttpResponse;
}

#[derive(Debug, Display, Error, Serialize, Clone)]
#[display(fmt = "Error: {}", message)]
pub struct NotFoundError {
    pub message: String,
}

impl InternalErrorTrait for NotFoundError {
    fn to_http_response(&self) -> HttpResponse {
        HttpResponse::NotFound().json(self) // Return 404 with the error message
    }
}

impl WebResponseError for NotFoundError {}

pub fn create_internal_error<T>(e: T) -> web::Error
where
    T: InternalErrorTrait + Debug + Display + 'static + Clone,
{
    InternalError::from_response(e.clone(), e.to_http_response()).into()
}
