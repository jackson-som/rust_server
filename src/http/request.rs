use std::str::from_utf8;
use std::str::Utf8Error;

use super::RequestError;
use super::RequestMethod;
use super::Query;

#[derive(Debug)]
pub struct Request<'buf> {
    method: RequestMethod,
    path: &'buf str,
    query: Option<Query<'buf>>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }
}

// Exp: GET /test?a=1&b=2 HTTP/1.1
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = RequestError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = from_utf8(buf)?;

        println!("Request : {}", request);

        let (method, request) = get_next_word(request).ok_or(RequestError::InvalidRequest)?;
        let (mut path_string, request) = get_next_word(request).ok_or(RequestError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(RequestError::InvalidRequest)?;

        println!("Method : {}", method); // GET
        println!("Path : {}", path_string); // /test?a=1&b=2
        println!("Protocal : {}", protocol); // HTTP/1.1

        if protocol != "HTTP/1.1" {
            return Err(RequestError::InvalidProtocol);
        }

        // Parse to enum for request method
        let request_method: RequestMethod = method.parse()?;

        let mut query_string = None;

        // Find index of '?' and make condition
        if let Some(i) = path_string.find('?') {
            query_string = Some(Query::from(&path_string[i + 1..])); // a=1&b=2 (Plus one for prevent '?')
            path_string = &path_string[..i]; // /test
        }

        Ok(Self {
            method: request_method,
            path: path_string,
            query: query_string,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, char_str) in request.chars().enumerate() {
        if char_str == ' ' || char_str == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl From<Utf8Error> for RequestError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
