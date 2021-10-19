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

use crate::routes::{
    api_insert_paste, api_language, api_languages, display_paste, index, insert_paste, raw_paste,
    run,
};
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::tera::{self, Value};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use std::collections::HashMap;

#[database("main")]
pub struct Db(PgConnection);

fn js_path(_: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    #[cfg(not(debug_assertions))]
    let path = env!("ENTRY_FILE_PATH");
    #[cfg(debug_assertions)]
    let path = std::fs::read_to_string("entry")?;
    Ok(path.into())
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::custom(|engines| {
            engines.tera.register_function("js_path", js_path);
        }))
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
        .mount(
            "/",
            routes![
                api_language,
                api_languages,
                api_insert_paste,
                run,
                index,
                insert_paste,
                display_paste,
                raw_paste,
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
}
