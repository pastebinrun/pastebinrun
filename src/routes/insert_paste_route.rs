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
use chrono::{Duration, Utc};
use rocket::form::Form;
use rocket::response::Redirect;

#[derive(FromForm)]
pub struct PasteForm {
    language: String,
    code: String,
    share: Share,
    #[field(default = "")]
    stdin: String,
    stdout: Option<String>,
    stderr: Option<String>,
    status: Option<i32>,
}

#[derive(FromFormField)]
pub enum Share {
    Share,
    Share24,
}

#[post("/", data = "<form>")]
pub async fn insert_paste(db: Db, form: Form<PasteForm>) -> Result<Redirect, InsertionError> {
    let delete_at = match form.share {
        Share::Share => None,
        Share::Share24 => Some(Utc::now() + Duration::hours(24)),
    };
    let identifier = db
        .run(move |conn| {
            paste::insert(
                conn,
                delete_at,
                &form.language,
                &form.code,
                ExtraPasteParameters {
                    stdin: &form.stdin,
                    stdout: form.stdout.as_deref(),
                    stderr: form.stderr.as_deref(),
                    exit_code: form.status,
                },
            )
        })
        .await?;
    Ok(Redirect::to(uri!(super::display_paste(identifier))))
}
