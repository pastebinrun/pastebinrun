// SPDX-FileCopyrightText: 2021 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    output: Option<String>,
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
                    output: form.output.as_deref(),
                    exit_code: form.status,
                },
            )
        })
        .await?;
    Ok(Redirect::to(uri!(super::display_paste(identifier))))
}
