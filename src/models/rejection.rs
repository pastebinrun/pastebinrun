use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use warp::http::StatusCode;

#[derive(Debug)]
pub enum CustomRejection {
    UnrecognizedLanguageIdentifier,
}

impl CustomRejection {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::UnrecognizedLanguageIdentifier => StatusCode::BAD_REQUEST,
        }
    }
}

impl Display for CustomRejection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::UnrecognizedLanguageIdentifier => write!(f, "unrecognized language identifier"),
        }
    }
}

impl Error for CustomRejection {}
