#[macro_use]
extern crate diesel;

mod schema;

use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::Database;
use actix_web::error::InternalError;
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::header::{
    CONTENT_SECURITY_POLICY, LOCATION, REFERRER_POLICY, X_FRAME_OPTIONS, X_XSS_PROTECTION,
};
use actix_web::http::{Method, StatusCode};
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{server, App, AsyncResponder, Form, HttpResponse, Path, State};
use askama::actix_web::TemplateIntoResponse;
use askama::Template;
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use futures::future::{self, Either};
use futures::prelude::*;
use pulldown_cmark::{html, Options, Parser};
use rand::prelude::*;
use schema::{languages, pastes};
use serde::de::IgnoredAny;
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
    autodelete: Option<IgnoredAny>,
}

#[derive(Insertable)]
#[table_name = "pastes"]
struct NewPaste {
    identifier: String,
    delete_at: Option<DateTime<Utc>>,
    language_id: i32,
    paste: String,
}

const CHARACTERS: &[u8] = b"23456789bcdfghjkmnpqrstvwxzBCDFGHJKLMNPQRSTVWX_-";

fn insert_paste(db: State<Database<PgConnection>>, Form(form): Form<PasteForm>) -> AsyncResponse {
    let mut rng = thread_rng();
    let identifier: String = (0..10)
        .map(|_| char::from(*CHARACTERS.choose(&mut rng).expect("a random character")))
        .collect();
    let delete_at = form.autodelete.map(|_| Utc::now() + Duration::hours(24));
    let cloned_identifier = identifier.clone();
    diesel::insert_into(pastes::table)
        .values(NewPaste {
            identifier,
            delete_at,
            language_id: form.language,
            paste: form.code,
        })
        .execute_async(&db)
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
struct DisplayPaste {
    languages: Vec<Language>,
    paste: Paste,
}

#[derive(Queryable)]
struct QueryPaste {
    paste: String,
    language_id: i32,
    delete_at: Option<DateTime<Utc>>,
    is_markdown: bool,
}

impl QueryPaste {
    fn into_paste(self) -> Paste {
        let QueryPaste {
            paste,
            language_id,
            delete_at,
            is_markdown,
        } = self;
        let markdown = if is_markdown {
            render_markdown(&paste)
        } else {
            String::new()
        };
        Paste {
            paste,
            language_id,
            delete_at,
            markdown,
        }
    }
}

struct Paste {
    paste: String,
    language_id: i32,
    delete_at: Option<DateTime<Utc>>,
    markdown: String,
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
                .inner_join(languages::table)
                .select((
                    pastes::paste,
                    pastes::language_id,
                    pastes::delete_at,
                    languages::is_markdown,
                ))
                .filter(pastes::identifier.eq(requested_identifier.into_inner()))
                .get_optional_result_async::<QueryPaste>(&db)
                .map(|paste| (db, paste))
        })
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR).into())
        .and_then(|(db, paste)| match paste {
            None => Either::A(future::ok(HttpResponse::NotFound().finish())),
            Some(paste) => Either::B(fetch_languages(&db).and_then(|languages| {
                DisplayPaste {
                    languages,
                    paste: paste.into_paste(),
                }
                .into_response()
            })),
        })
        .responder()
}

fn render_markdown(markdown: &str) -> String {
    let mut output = String::new();
    html::push_html(
        &mut output,
        Parser::new_ext(markdown, Options::ENABLE_TABLES),
    );
    ammonia::clean(&output)
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
            .middleware(
                DefaultHeaders::new()
                    .header(
                        CONTENT_SECURITY_POLICY,
                        concat!(
                            "default-src 'none'; ",
                            "script-src 'self'; ",
                            "style-src 'self'; ",
                            "img-src *; ",
                            "object-src 'none'; ",
                            "base-uri 'none'; ",
                            "form-action 'self'; ",
                            "frame-ancestors 'none'",
                        ),
                    )
                    .header(X_FRAME_OPTIONS, "DENY")
                    .header(X_XSS_PROTECTION, "1; mode=block")
                    .header(REFERRER_POLICY, "no-referrer"),
            )
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
