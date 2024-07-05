use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
enum StatusCode {
    Ok = 200,
    BadRequest  = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) ->&str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad request",
            Self::NotFound => "Not found"
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.clone() as u16)
    }
}

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body,
        }
    }
}