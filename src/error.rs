use ntex::http::StatusCode;
use ntex::web::{self, HttpRequest, WebResponseError};
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize)]
pub struct ApplicationError {
    pub message: String,
    pub status: u16,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl WebResponseError for ApplicationError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self, _: &HttpRequest) -> web::HttpResponse {
        let err_json = json!({ "message": self.message });
        web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(&err_json)
    }
}
