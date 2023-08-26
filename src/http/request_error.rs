#![allow(clippy::enum_variant_names)]

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
#[non_exhaustive]
pub enum RequestError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use RequestError::*;

        match self {
            InvalidRequest => write!(f, "Invalid request"),
            InvalidEncoding => write!(f, "Failed encoding"),
            InvalidProtocol => write!(f, "Invalid protocol"),
            InvalidMethod => write!(f, "Invalid request method"),
        }
    }
}
