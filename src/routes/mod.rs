mod api_language;
mod api_v1;
mod display_paste;
mod index;
mod insert_paste;
mod raw_paste;
mod run;

use crate::templates::{self, RenderRucte};
use crate::PgPool;
use std::ffi::OsStr;
use std::path::PathBuf;
use warp::http::header::{
    HeaderMap, HeaderValue, CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_FRAME_OPTIONS,
};
use warp::http::{Response, StatusCode};
use warp::{path, Filter, Rejection, Reply};

pub fn routes(pool: &'static PgPool) -> impl Filter<Extract = (impl Reply,), Error = Rejection> {
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
    let run = path!("api" / "v0" / "run" / i32)
        .and(warp::path::end())
        .and(warp::post2())
        .and(warp::body::content_length_limit(1_000_000))
        .and(warp::body::form())
        .and(pool)
        .and_then(run::run);
    let static_dir = warp::path("static").and(warp::fs::dir("static"));
    let favicon = warp::path("favicon.ico")
        .and(warp::path::end())
        .and(warp::fs::file("static/favicon.ico"));
    let api_v1_languages = path!("api" / "v1")
        .and(warp::path("languages"))
        .and(warp::path::end())
        .and(warp::get2())
        .and(pool)
        .and_then(api_v1::languages::languages);
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
        .or(api_v1_languages)
        .or(run)
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
    use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
    use diesel::Connection;
    use lazy_static::lazy_static;
    use scraper::{Html, Selector};
    use serde::Deserialize;
    use std::env;
    use std::str;
    use warp::filters::BoxedFilter;
    use warp::http::header::{CONTENT_LENGTH, LOCATION};
    use warp::reply::{Reply, Response};
    use warp::Filter;

    lazy_static! {
        static ref POOL: PgPool = {
            let pool = Pool::builder()
                .connection_customizer(Box::new(ExecuteWithinTransaction))
                .max_size(1)
                .build(ConnectionManager::new(env::var("DATABASE_URL").expect(
                    "Setting DATABASE_URL environment variable required to run tests",
                )))
                .expect("Couldn't create a connection pool");
            diesel_migrations::run_pending_migrations(&pool.get().unwrap()).unwrap();
            pool
        };
        static ref ROUTES: BoxedFilter<(Response,)> =
            routes(&POOL).map(Reply::into_response).boxed();
    }

    #[derive(Debug)]
    struct ExecuteWithinTransaction;

    impl<C, E> CustomizeConnection<C, E> for ExecuteWithinTransaction
    where
        C: Connection,
    {
        fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
            conn.begin_test_transaction().unwrap();
            Ok(())
        }
    }

    fn get_html_id() -> String {
        let response = warp::test::request().reply(&*ROUTES);
        let document = Html::parse_document(str::from_utf8(response.body()).unwrap());
        document
            .select(&Selector::parse("#language option").unwrap())
            .find(|element| element.text().next() == Some("HTML"))
            .expect("a language called HTML to exist")
            .value()
            .attr("value")
            .expect("an ID")
            .to_string()
    }

    #[test]
    fn test_language_api() {
        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ApiLanguage<'a> {
            mode: &'a str,
            mime: &'a str,
        }
        let response = warp::test::request()
            .path(&format!("/api/v0/language/{}", get_html_id()))
            .reply(&*ROUTES);
        assert_eq!(
            serde_json::from_slice::<ApiLanguage>(response.body()).unwrap(),
            ApiLanguage {
                mode: "htmlmixed",
                mime: "text/html"
            },
        );
    }

    #[test]
    fn test_raw_pastes() {
        let body = format!("language={}&code=abc", get_html_id());
        let reply = warp::test::request()
            .method("POST")
            .header(CONTENT_LENGTH, body.len())
            .body(body)
            .reply(&*ROUTES);
        let location = reply.headers()[LOCATION].to_str().unwrap();
        assert_eq!(
            warp::test::request()
                .path(&format!("{}.txt", location))
                .reply(&*ROUTES)
                .body(),
            "abc"
        );
    }
}
