use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use warp::http::StatusCode;

#[derive(Debug)]
pub enum CustomRejection {
    UnrecognizedLanguageIdentifier,
    FieldTooLarge(&'static str),
}

impl CustomRejection {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::UnrecognizedLanguageIdentifier => StatusCode::BAD_REQUEST,
            Self::FieldTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
        }
    }
}

impl Display for CustomRejection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::UnrecognizedLanguageIdentifier => write!(f, "unrecognized language identifier"),
            Self::FieldTooLarge(name) => write!(f, "{} is longer than a megabyte", name),
        }
    }
}

impl Error for CustomRejection {}
