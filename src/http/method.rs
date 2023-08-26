use super::RequestError;
use std::str::FromStr;

#[derive(Debug)]
pub enum RequestMethod {
    Post,
    Get,
    Put,
    Delete,
    Head,
    Connect,
    Patch,
    Trace,
    Options,
}

impl FromStr for RequestMethod {
    type Err = RequestError;

    fn from_str(method: &str) -> Result<Self, Self::Err> {
        match method {
            "POST" => Ok(Self::Post),
            "GET" => Ok(Self::Get),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "HEAD" => Ok(Self::Head),
            "CONNECT" => Ok(Self::Connect),
            "PATCH" => Ok(Self::Patch),
            "TRACE" => Ok(Self::Trace),
            "OPTIONS" => Ok(Self::Options),
            _ => Err(RequestError::InvalidMethod),
        }
    }
}
