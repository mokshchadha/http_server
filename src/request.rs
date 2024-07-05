use crate::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::{self, Utf8Error};


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buff)?;
        let (method, remaining) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, remaining) = get_next_word(remaining).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(remaining).ok_or(ParseError::InvalidRequest)?;

        if (protocol != "HTTP/1.1") {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(path[i + 1..].to_string());
            path = &path[..i];
        }

        Ok(Self{
            path : path.to_string(), query_string, method
        })
    }
}


fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,

}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidRequest",
            Self::InvalidProtocol => "InvalidRequest",
            Self::InvalidMethod => "InvalidRequest",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}