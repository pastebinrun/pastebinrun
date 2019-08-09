mod api_language;
mod display_paste;
mod index;
mod insert_paste;
mod raw_paste;

use crate::templates::{self, RenderRucte};
use crate::PgPool;
use std::ffi::OsStr;
use std::path::PathBuf;
use warp::http::header::{
    HeaderMap, HeaderValue, CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_FRAME_OPTIONS,
};
use warp::http::{Response, StatusCode};
use warp::{path, Filter, Rejection, Reply};

pub fn routes(pool: &'static PgPool) -> impl Filter<Extract = (impl Reply,)> {
    let pool = warp::any().map(move || pool);
    let index = warp::path::end()
        .and(warp::get2())
        .and(pool)
        .and_then(index::index);
    let display_paste = warp::path::param()
        .and(warp::path::end())
        .and(warp::get2())
        .and(pool)
        .and_then(display_paste::display_paste);
    let raw_paste = with_ext("txt")
        .and(warp::get2())
        .and(pool)
        .and_then(raw_paste::raw_paste);
    let insert_paste = warp::path::end()
        .and(warp::post2())
        .and(warp::body::content_length_limit(1_000_000))
        .and(warp::body::form())
        .and(pool)
        .and_then(insert_paste::insert_paste);
    let api_language = path!("api" / "v0" / "language" / i32)
        .and(warp::path::end())
        .and(warp::get2())
        .and(pool)
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
    headers.insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    index
        .or(favicon)
        .or(raw_paste)
        .or(display_paste)
        .or(insert_paste)
        .or(api_language)
        .or(static_dir)
        .recover(not_found)
        .with(warp::reply::with::headers(headers))
        .with(warp::log("pastebinrun"))
}

fn with_ext(ext: &'static str) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::path::param()
        .and(warp::path::end())
        .and_then(move |path: PathBuf| {
            match (path.extension(), path.file_stem().and_then(OsStr::to_str)) {
                (Some(received_ext), Some(file_stem)) if ext == received_ext => {
                    Ok(file_stem.to_string())
                }
                _ => Err(warp::reject::not_found()),
            }
        })
}

fn not_found(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if rejection.is_not_found() {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .html(|o| templates::not_found(o))
    } else {
        Err(rejection)
    }
}

#[cfg(test)]
mod test {
    use super::routes;
    use crate::PgPool;
    use diesel::r2d2::{ConnectionManager, Pool};
    use lazy_static::lazy_static;
    use scraper::{Html, Selector};
    use serde::Deserialize;
    use std::env;
    use std::str;

    lazy_static! {
        static ref POOL: PgPool = {
            let pool = Pool::new(ConnectionManager::new(
                env::var("DATABASE_URL")
                    .expect("Setting DATABASE_URL environment variable required to run tests"),
            ))
            .expect("Couldn't create a connection pool");
            diesel_migrations::run_pending_migrations(&pool.get().unwrap()).unwrap();
            pool
        };
    }

    #[test]
    fn test_language_api() {
        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ApiLanguage<'a> {
            mode: &'a str,
            mime: &'a str,
        }
        let routes = routes(&POOL);
        let response = warp::test::request().reply(&routes);
        let document = Html::parse_document(str::from_utf8(response.body()).unwrap());
        let id = document
            .select(&Selector::parse("#language option").unwrap())
            .find(|element| element.text().next() == Some("HTML"))
            .expect("a language called HTML to exist")
            .value()
            .attr("value")
            .expect("an ID");
        let response = warp::test::request()
            .path(&format!("/api/v0/language/{}", id))
            .reply(&routes);
        assert_eq!(
            serde_json::from_slice::<ApiLanguage>(response.body()).unwrap(),
            ApiLanguage {
                mode: "htmlmixed",
                mime: "text/html"
            },
        );
    }
}
