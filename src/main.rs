// pastebin.run
// Copyright (C) 2020-2021 Konrad Borowski
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

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod migration;
mod models;
mod routes;
mod schema;

use crate::models::language::Language;
use crate::models::paste::{self, ExtraPasteParameters, InsertionError, Paste};
use crate::routes::index;
use crate::schema::{languages, pastes};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::http::impl_from_uri_param_identity;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Path, UriDisplay};
use rocket::http::uri::Origin;
use rocket::request::FromParam;
use rocket::response::{Debug, Redirect};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use serde::Serialize;
use std::fmt;

#[database("main")]
pub struct Db(PgConnection);

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
async fn insert_paste(db: Db, form: Form<PasteForm>) -> Result<Redirect, InsertionError> {
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
    Ok(Redirect::to(uri!(display_paste(identifier))))
}

#[derive(Serialize)]
struct DisplayPaste {
    languages: Vec<Language>,
    description: String,
    paste: String,
    selected_id: i32,
    raw_paste_url: Origin<'static>,
}

#[get("/<identifier>", rank = 2)]
async fn display_paste(
    db: Db,
    identifier: String,
) -> Result<Option<Template>, Debug<diesel::result::Error>> {
    db.run(move |conn| {
        Paste::delete_old(conn)?;
        let languages = Language::fetch(conn)?;
        let paste: Option<Paste> = pastes::table
            .inner_join(languages::table.on(pastes::language_id.eq(languages::language_id)))
            .select((
                pastes::identifier,
                pastes::paste,
                pastes::language_id,
                pastes::delete_at,
                languages::identifier,
                pastes::stdin,
                pastes::exit_code,
                pastes::stdout,
                pastes::stderr,
            ))
            .filter(pastes::identifier.eq(&identifier))
            .get_result(conn)
            .optional()?;
        if let Some(paste) = paste {
            let description = generate_description(&paste.paste);
            Ok(Some(Template::render(
                "display-paste",
                &DisplayPaste {
                    languages,
                    description,
                    paste: paste.paste,
                    selected_id: paste.language_id,
                    raw_paste_url: uri!(raw_paste(identifier)),
                },
            )))
        } else {
            Ok(None)
        }
    })
    .await
}

fn generate_description(paste: &str) -> String {
    let mut description = paste.chars().take(239).collect();
    if description != paste {
        description += "…";
    }
    description
}

struct WithTxt(String);

impl<'a> FromParam<'a> for WithTxt {
    type Error = &'a str;

    fn from_param(param: &str) -> Result<Self, &str> {
        if let Some(param) = param.strip_suffix(".txt") {
            Ok(WithTxt(String::from_param(param)?))
        } else {
            Err(param)
        }
    }
}

impl UriDisplay<Path> for WithTxt {
    fn fmt(&self, f: &mut Formatter<Path>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_raw(".txt")
    }
}

impl_from_uri_param_identity!([Path] WithTxt);

impl FromUriParam<Path, String> for WithTxt {
    type Target = WithTxt;

    fn from_uri_param(param: String) -> WithTxt {
        WithTxt(param.to_string())
    }
}

#[get("/<identifier>")]
async fn raw_paste(
    db: Db,
    identifier: WithTxt,
) -> Result<Option<String>, Debug<diesel::result::Error>> {
    db.run(move |conn| {
        Paste::delete_old(conn)?;
        Ok(pastes::table
            .select(pastes::paste)
            .filter(pastes::identifier.eq(&identifier.0))
            .get_result(conn)
            .optional()?)
    })
    .await
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(Db::fairing())
        .attach(AdHoc::on_ignite("Migrations", |rocket| async {
            Db::get_one(&rocket)
                .await
                .expect("a database")
                .run(|conn| {
                    diesel_migrations::run_pending_migrations(conn)?;
                    migration::run(conn)
                })
                .await
                .expect("database to be migrated");
            rocket
        }))
        .mount("/", routes![index, insert_paste, display_paste, raw_paste])
        .mount("/static", FileServer::from(relative!("static")))
}
