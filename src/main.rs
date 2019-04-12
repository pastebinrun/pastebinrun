#[macro_use]
extern crate diesel;

mod schema;

use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::{AsyncError, Database};
use actix_web::error::InternalError;
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::header::{
    CACHE_CONTROL, CONTENT_SECURITY_POLICY, LOCATION, REFERRER_POLICY, X_FRAME_OPTIONS,
    X_XSS_PROTECTION,
};
use actix_web::http::{Method, StatusCode};
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{server, App, AsyncResponder, Form, HttpResponse, Path, State};
use ammonia::Builder;
use askama::actix_web::TemplateIntoResponse;
use askama::Template;
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use futures::future;
use futures::prelude::*;
use lazy_static::lazy_static;
use log::info;
use pulldown_cmark::{html, Options, Parser};
use rand::prelude::*;
use schema::{languages, pastes};
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};
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
        .select((languages::language_id, languages::name))
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
    no_follow: bool,
}

#[derive(Template)]
#[template(path = "404.html")]
struct PasteNotFound;

impl QueryPaste {
    fn into_paste(self) -> Paste {
        let QueryPaste {
            paste,
            language_id,
            delete_at,
            is_markdown,
            no_follow,
        } = self;
        let markdown = if is_markdown {
            render_markdown(&paste, no_follow)
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
    delete_old_pastes(&db)
        .and_then(|_| {
            pastes::table
                .inner_join(languages::table)
                .select((
                    pastes::paste,
                    pastes::language_id,
                    pastes::delete_at,
                    languages::is_markdown,
                    pastes::no_follow,
                ))
                .filter(pastes::identifier.eq(requested_identifier.into_inner()))
                .get_optional_result_async::<QueryPaste>(&db)
                .map(|paste| (db, paste))
        })
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR).into())
        .and_then(|(db, paste)| match paste {
            None => future::Either::A(future::ok(
                HttpResponse::NotFound().body(PasteNotFound.render().unwrap()),
            )),
            Some(paste) => future::Either::B(fetch_languages(&db).and_then(|languages| {
                DisplayPaste {
                    languages,
                    paste: paste.into_paste(),
                }
                .into_response()
            })),
        })
        .responder()
}

fn delete_old_pastes(
    db: &Database<PgConnection>,
) -> impl Future<Item = (), Error = AsyncError<diesel::result::Error>> {
    diesel::delete(pastes::table)
        .filter(pastes::delete_at.lt(Utc::now()))
        .execute_async(&db)
        .map(|pastes| {
            if pastes > 0 {
                info!("Deleted {} paste(s)", pastes);
            }
        })
}

fn render_markdown(markdown: &str, no_follow: bool) -> String {
    lazy_static! {
        static ref FILTER: Builder<'static> = {
            let mut builder = Builder::new();
            builder.link_rel(Some("noopener noreferrer nofollow"));
            builder
        };
    }
    let mut output = String::new();
    html::push_html(
        &mut output,
        Parser::new_ext(markdown, Options::ENABLE_TABLES),
    );
    if no_follow {
        FILTER.clean(&output).to_string()
    } else {
        ammonia::clean(&output)
    }
}

fn raw(db: State<Database<PgConnection>>, requested_identifier: Path<String>) -> AsyncResponse {
    delete_old_pastes(&db)
        .and_then(move |_| {
            pastes::table
                .select(pastes::paste)
                .filter(pastes::identifier.eq(requested_identifier.into_inner()))
                .get_optional_result_async::<String>(&db)
        })
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR).into())
        .map(|paste| match paste {
            None => HttpResponse::NotFound().finish(),
            Some(paste) => HttpResponse::Ok().content_type("text/plain").body(paste),
        })
        .responder()
}

fn favicon(_: ()) -> io::Result<NamedFile> {
    NamedFile::open("static/favicon.ico")
}

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
struct ApiLanguage {
    mode: Option<String>,
    mime: String,
}

fn api_language(db: State<Database<PgConnection>>, id: Path<i32>) -> AsyncResponse {
    languages::table
        .find(id.into_inner())
        .select((languages::highlighter_mode, languages::mime))
        .get_optional_result_async(&db)
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR).into())
        .map(|json: Option<ApiLanguage>| match json {
            Some(json) => HttpResponse::Ok()
                .header(CACHE_CONTROL, "max-age=14400")
                .json(json),
            None => HttpResponse::NotFound().finish(),
        })
        .responder()
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
                            "default-src 'self'; ",
                            "img-src *; ",
                            "object-src 'none'; ",
                            "base-uri 'none'; ",
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
            .resource("/{identifier}/raw", |r| r.method(Method::GET).with(raw))
            .resource("/api/v0/language/{id}", |r| {
                r.method(Method::GET).with(api_language)
            })
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(())
}
