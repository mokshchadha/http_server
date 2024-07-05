use std::fmt::{Display, Formatter};
use std::io::{Write, Result};
use std::net::TcpStream;

#[derive(Copy, Clone)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad request",
            Self::NotFound => "Not found"
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body,
        }
    }


    pub fn send(&self, stream: &mut TcpStream) -> Result<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
    }
}