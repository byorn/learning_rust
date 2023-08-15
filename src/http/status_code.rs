use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::OK => "OK",
            StatusCode::BadRequest => "Bad request",
            StatusCode::NotFound =>  "Not found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
       write!(f, "{}", *self as u16)
    }
}