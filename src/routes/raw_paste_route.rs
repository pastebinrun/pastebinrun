// pastebin.run
// Copyright (C) 2021 Konrad Borowski
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

use crate::models::paste::Paste;
use crate::schema::pastes;
use crate::Db;
use diesel::prelude::*;
use rocket::http::hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Path, UriDisplay};
use rocket::http::{impl_from_uri_param_identity, Header};
use rocket::request::{FromParam, Request};
use rocket::response::status::NotFound;
use rocket::response::{self, Debug, Responder, Response};
use std::fmt;

pub struct WithTxt<'a>(&'a str);

impl<'a> FromParam<'a> for WithTxt<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, &str> {
        param
            .strip_suffix(".txt")
            .map_or(Err(param), |param| Ok(WithTxt(param)))
    }
}

impl UriDisplay<Path> for WithTxt<'_> {
    fn fmt(&self, f: &mut Formatter<Path>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_raw(".txt")
    }
}

impl_from_uri_param_identity!([Path] ('a) WithTxt<'a>);

impl<'a> FromUriParam<Path, &'a str> for WithTxt<'a> {
    type Target = WithTxt<'a>;

    fn from_uri_param(param: &'a str) -> WithTxt<'a> {
        WithTxt(param)
    }
}

pub enum RawPasteResponse {
    Found(String),
    NotFound,
}

const PASTE_NOT_FOUND_MESSAGE: &str =
    "404 Paste Not Found\n\nIt could have been automatically deleted after 24 hours.\n";

impl<'r> Responder<'r, 'static> for RawPasteResponse {
    fn respond_to(self, r: &Request<'_>) -> response::Result<'static> {
        Response::build()
            .merge(if let Self::Found(paste) = self {
                paste.respond_to(r)
            } else {
                NotFound(PASTE_NOT_FOUND_MESSAGE).respond_to(r)
            }?)
            .header(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*"))
            .ok()
    }
}

#[get("/<identifier>")]
pub async fn raw_paste(
    db: Db,
    identifier: WithTxt<'_>,
) -> Result<RawPasteResponse, Debug<diesel::result::Error>> {
    let identifier = identifier.0.to_string();
    Ok(db
        .run(move |conn| {
            Paste::delete_old(conn)?;
            pastes::table
                .select(pastes::paste)
                .filter(pastes::identifier.eq(identifier))
                .get_result(conn)
                .optional()
        })
        .await?
        .map_or_else(|| RawPasteResponse::NotFound, RawPasteResponse::Found))
}
