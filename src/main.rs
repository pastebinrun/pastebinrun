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

use crate::models::paste::Paste;
use crate::routes::{display_paste, index, insert_paste};
use crate::schema::pastes;
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket::http::impl_from_uri_param_identity;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Path, UriDisplay};
use rocket::request::FromParam;
use rocket::response::Debug;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use std::fmt;

#[database("main")]
pub struct Db(PgConnection);

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
        WithTxt(param)
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
