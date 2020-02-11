// pastebin.run
// Copyright (C) 2020 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt::{Display, Formatter, Result};
use warp::http::StatusCode;
use warp::reject::Reject;

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

impl Reject for CustomRejection {}
