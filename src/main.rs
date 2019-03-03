#[macro_use]
extern crate diesel;

mod schema;

use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::Database;
use actix_web::error::InternalError;
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::header::LOCATION;
use actix_web::http::{Method, StatusCode};
use actix_web::middleware::Logger;
use actix_web::{server, App, AsyncResponder, Form, HttpResponse, Path, State};
use askama::actix_web::TemplateIntoResponse;
use askama::Template;
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use futures::future::{self, Either};
use futures::prelude::*;
use rand::prelude::*;
use schema::{languages, paste_contents, paste_revisions, pastes};
use serde::Deserialize;
use std::{env, io};

type AsyncResponse = Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>>;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    languages: Vec<Language>,
}

#[derive(Queryable)]
struct Language {
    id: i32,
    name: String,
    highlighter_mode: Option<String>,
    mime: String,
}

fn index(db: State<Database<PgConnection>>) -> AsyncResponse {
    fetch_languages(&db)
        .and_then(|languages| Index { languages }.into_response())
        .responder()
}

fn fetch_languages(
    db: &Database<PgConnection>,
) -> impl Future<Item = Vec<Language>, Error = actix_web::Error> {
    languages::table
        .select((
            languages::language_id,
            languages::name,
            languages::highlighter_mode,
            languages::mime,
        ))
        .order((languages::priority.asc(), languages::name.asc()))
        .load_async(&db)
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR).into())
}

#[derive(Deserialize)]
struct PasteForm {
    language: i32,
    code: String,
    autodelete: Checkbox,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Checkbox {
    On,
    Off,
}

#[derive(Insertable)]
#[table_name = "pastes"]
struct NewPaste {
    identifier: String,
    delete_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name = "paste_revisions"]
struct NewPasteRevision {
    paste_id: i32,
}

#[derive(Insertable)]
#[table_name = "paste_contents"]
struct NewPasteContent {
    paste_revision_id: i32,
    language_id: i32,
    paste: String,
}

const CHARACTERS: &[u8] = b"23456789bcdfghjkmnpqrstvwxzBCDFGHJKLMNPQRSTVWX_-";

fn insert_paste(db: State<Database<PgConnection>>, Form(form): Form<PasteForm>) -> AsyncResponse {
    let mut rng = thread_rng();
    let identifier: String = (0..24)
        .map(|_| char::from(*CHARACTERS.choose(&mut rng).expect("a random character")))
        .collect();
    let delete_at = match form.autodelete {
        Checkbox::On => Some(Utc::now() + Duration::hours(24)),
        Checkbox::Off => None,
    };
    let cloned_identifier = identifier.clone();
    db.transaction(move |c| {
        let paste_id = diesel::insert_into(pastes::table)
            .values(NewPaste {
                identifier,
                delete_at,
            })
            .returning(schema::pastes::columns::paste_id)
            .get_result(c)?;
        let paste_revision_id = diesel::insert_into(paste_revisions::table)
            .values(NewPasteRevision { paste_id })
            .returning(schema::paste_revisions::columns::paste_revision_id)
            .get_result(c)?;
        diesel::insert_into(paste_contents::table)
            .values(NewPasteContent {
                paste_revision_id,
                language_id: form.language,
                paste: form.code,
            })
            .execute(c)
    })
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR).into())
    .map(move |_| {
        HttpResponse::Found()
            .header(LOCATION, format!("/{}", cloned_identifier))
            .finish()
    })
    .responder()
}

#[derive(Template)]
#[template(path = "viewpaste.html")]
struct DisplayPastes {
    languages: Vec<Language>,
    pastes: Vec<DisplayPaste>,
}

#[derive(Queryable)]
struct DisplayPaste {
    paste: String,
    language_id: i32,
    delete_at: Option<DateTime<Utc>>,
}

fn display_paste(
    db: State<Database<PgConnection>>,
    requested_identifier: Path<String>,
) -> AsyncResponse {
    diesel::delete(pastes::table)
        .filter(pastes::delete_at.lt(Utc::now()))
        .execute_async(&db)
        .and_then(|_| {
            pastes::table
                .inner_join(paste_revisions::table.inner_join(paste_contents::table))
                .select((
                    paste_contents::paste,
                    paste_contents::language_id,
                    pastes::delete_at,
                ))
                .filter(
                    paste_revisions::table
                        .filter(pastes::identifier.eq(requested_identifier.into_inner()))
                        .order(paste_revisions::created_at.desc())
                        .select(paste_revisions::paste_revision_id)
                        .single_value()
                        .eq(paste_contents::paste_revision_id.nullable()),
                )
                .load_async(&db)
                .map(|pastes| (db, pastes))
        })
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR).into())
        .and_then(|(db, pastes)| {
            if pastes.is_empty() {
                Either::A(future::ok(HttpResponse::NotFound().finish()))
            } else {
                Either::B(
                    fetch_languages(&db)
                        .and_then(|languages| DisplayPastes { languages, pastes }.into_response()),
                )
            }
        })
        .responder()
}

fn favicon(_: ()) -> io::Result<NamedFile> {
    NamedFile::open("static/favicon.ico")
}

fn main() -> io::Result<()> {
    env_logger::init();
    let db = Database::open(env::var("DATABASE_URL").expect("DATABASE_URL required"));
    server::new(move || {
        App::with_state(db.clone())
            .middleware(Logger::default())
            .resource("/", |r| {
                r.method(Method::GET).with(index);
                r.method(Method::POST).with(insert_paste);
            })
            .resource("/favicon.ico", |r| r.method(Method::GET).with(favicon))
            .handler("/static", StaticFiles::new("static").unwrap())
            .resource("/{identifier}", |r| {
                r.method(Method::GET).with(display_paste)
            })
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(())
}
