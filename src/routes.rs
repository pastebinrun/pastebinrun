mod api_language;
mod display_paste;
mod index;
mod insert_paste;
mod raw_paste;

use crate::render;
use askama::Template;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use warp::http::header::{
    HeaderMap, HeaderValue, CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_FRAME_OPTIONS,
    X_XSS_PROTECTION,
};
use warp::http::StatusCode;
use warp::{path, Filter, Rejection, Reply};

#[derive(Template)]
#[template(path = "404.html")]
struct NotFound;

pub fn routes() -> impl Filter<Extract = (impl Reply,)> {
    let pool = Pool::new(ConnectionManager::new(
        env::var("DATABASE_URL").expect("DATABASE_URL required"),
    ))
    .expect("Couldn't create a connection pool");
    let db = warp::any().map(move || pool.get().unwrap());
    let index = warp::path::end()
        .and(warp::get2())
        .and(db.clone())
        .map(index::index);
    let display_paste = warp::path::param()
        .and(warp::path::end())
        .and(warp::get2())
        .and(db.clone())
        .and_then(display_paste::display_paste);
    let raw_paste = path!(String / "raw")
        .and(warp::path::end())
        .and(warp::get2())
        .and(db.clone())
        .and_then(raw_paste::raw_paste);
    let insert_paste = warp::path::end()
        .and(warp::post2())
        .and(warp::body::content_length_limit(1_000_000))
        .and(warp::body::form())
        .and(db.clone())
        .map(insert_paste::insert_paste);
    let api_language = path!("api" / "v0" / "language" / i32)
        .and(warp::path::end())
        .and(warp::get2())
        .and(db)
        .and_then(api_language::api_language);
    let static_dir = warp::path("static").and(warp::fs::dir("static"));
    let favicon = warp::path("favicon.ico")
        .and(warp::path::end())
        .and(warp::fs::file("static/favicon.ico"));
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(concat!(
            "default-src 'none'; ",
            "script-src 'self'; ",
            "style-src 'self'; ",
            "connect-src 'self'; ",
            "img-src *; ",
            "object-src 'none'; ",
            "base-uri 'none'; ",
            "form-action 'self'; ",
            "frame-ancestors 'none'",
        )),
    );
    headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(X_XSS_PROTECTION, HeaderValue::from_static("1; mode=block"));
    headers.insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    index
        .or(favicon)
        .or(display_paste)
        .or(raw_paste)
        .or(insert_paste)
        .or(api_language)
        .or(static_dir)
        .recover(not_found)
        .with(warp::reply::with::headers(headers))
        .with(warp::log("pastebinrun"))
}

fn not_found(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if rejection.is_not_found() {
        Ok(warp::reply::with_status(
            render(NotFound),
            StatusCode::NOT_FOUND,
        ))
    } else {
        Err(rejection)
    }
}
