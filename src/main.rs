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
    api_insert_paste, api_language, api_languages, config, display_paste, index, insert_paste,
    metrics, raw_paste, run,
};
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket::shield::{Policy, Referrer, Shield};
use rocket_dyn_templates::tera::{self, Value};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use std::collections::HashMap;

#[database("main")]
pub struct Db(PgConnection);

fn js_path(_: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    #[cfg(not(debug_assertions))]
    let path = concat!("/", env!("ENTRY_FILE_PATH"));
    #[cfg(debug_assertions)]
    let path = "http://localhost:3000/js/index.ts";
    Ok(path.into())
}

fn css_stylesheet(_: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    #[cfg(not(debug_assertions))]
    let path = concat!("<link rel=stylesheet href='/", env!("CSS_PATH"), "'>");
    #[cfg(debug_assertions)]
    let path = "";
    Ok(path.into())
}

#[derive(Default)]
struct ContentSecurityPolicy;

impl Policy for ContentSecurityPolicy {
    const NAME: &'static str = "Content-Security-Policy";
    fn header(&self) -> Header<'static> {
        const CONTENT_SECURITY_POLICY: &str = if cfg!(debug_assertions) {
            concat!(
                "default-src 'none';",
                "script-src 'self' localhost:3000;",
                "style-src 'unsafe-inline';",
                "img-src data: https:;",
                "connect-src 'self' ws://localhost:3000;",
                "sandbox allow-forms allow-scripts allow-same-origin;",
                "form-action 'self';",
                "frame-ancestors 'none';",
                "base-uri 'none';",
                "worker-src 'none';",
                "manifest-src 'none'",
            )
        } else {
            concat!(
                "default-src 'none';",
                "script-src 'self';",
                "style-src 'self' 'unsafe-inline';",
                "img-src data: https:;",
                "connect-src 'self';",
                "sandbox allow-forms allow-scripts allow-same-origin;",
                "form-action 'self';",
                "frame-ancestors 'none';",
                "base-uri 'none';",
                "worker-src 'none';",
                "manifest-src 'none'",
            )
        };
        Header::new(Self::NAME, CONTENT_SECURITY_POLICY)
    }
}

#[launch]
fn rocket() -> _ {
    let mut rocket = rocket::build()
        .attach(Template::custom(|engines| {
            engines.tera.register_function("js_path", js_path);
            engines
                .tera
                .register_function("css_stylesheet", css_stylesheet);
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
        .attach(
            Shield::default()
                .enable(ContentSecurityPolicy)
                .enable(Referrer::NoReferrer),
        )
        .mount(
            "/",
            routes![
                api_language,
                api_languages,
                api_insert_paste,
                config,
                run,
                index,
                insert_paste,
                display_paste,
                raw_paste,
                metrics,
            ],
        );
    if cfg!(not(debug_assertions)) {
        rocket = rocket.mount("/assets", rocket::fs::FileServer::from("dist/assets"));
    }
    rocket
}
