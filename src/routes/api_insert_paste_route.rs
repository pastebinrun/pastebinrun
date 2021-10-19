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

use crate::models::paste::{self, ExtraPasteParameters, InsertionError};
use crate::Db;
use chrono::Duration;
use chrono::Utc;
use rocket::form::{self, Form, FromFormField, Strict, ValueField};
use rocket::http::hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use rocket::http::Header;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::error::Error;

#[derive(FromForm)]
pub struct PasteForm {
    #[field(default = Expiration(None))]
    expiration: Expiration,
    #[field(default = "plaintext")]
    language: String,
    code: String,
}

struct Expiration(Option<Duration>);

impl<'r> FromFormField<'r> for Expiration {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let duration = time_parse::duration::parse_nom(field.value).map_err(Box::from)?;
        let duration =
            Duration::from_std(duration).map_err(|x| -> Box<dyn Error + Send> { Box::new(x) })?;
        Ok(Self(Some(duration)))
    }
}

pub struct CorsString(String);

impl<'r> Responder<'r, 'static> for CorsString {
    fn respond_to(self, r: &Request<'_>) -> response::Result<'static> {
        Response::build()
            .merge(self.0.respond_to(r)?)
            .header(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*"))
            .ok()
    }
}

#[post("/api/v1/pastes", data = "<form>")]
pub async fn api_insert_paste(
    db: Db,
    form: Form<Strict<PasteForm>>,
) -> Result<CorsString, InsertionError> {
    let identifier = db
        .run(move |conn| {
            paste::insert(
                conn,
                form.expiration.0.map(|expiration| Utc::now() + expiration),
                &form.language,
                &form.code,
                ExtraPasteParameters {
                    stdin: "",
                    stdout: None,
                    stderr: None,
                    exit_code: None,
                },
            )
        })
        .await?;
    Ok(CorsString(identifier))
}
