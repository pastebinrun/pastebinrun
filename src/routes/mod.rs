// pastebin.run
// Copyright (C) 2020 Konrad Borowski
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

mod api_language;
mod api_v1;
mod config;
mod display_paste;
mod index;
mod insert_paste;
mod raw_paste;
mod run;

use crate::models::rejection::CustomRejection;
use crate::models::session::Session;
use crate::templates::{self, RenderRucte};
use crate::Connection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use futures03::compat::Compat;
use futures03::{Future, FutureExt, TryFutureExt};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::pin::Pin;
use tokio_executor::blocking;
use warp::filters::cors::Cors;
use warp::filters::BoxedFilter;
use warp::http::header::{
    HeaderMap, HeaderValue, CONTENT_SECURITY_POLICY, CONTENT_TYPE, REFERRER_POLICY, X_FRAME_OPTIONS,
};
use warp::http::method::Method;
use warp::http::{Response, StatusCode};
use warp::{path, Filter, Rejection, Reply};

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn connection(pool: PgPool) -> BoxedFilter<(Connection,)> {
    warp::any()
        .and_then(move || get_connection(pool.clone()).compat())
        .boxed()
}

fn get_connection(pool: PgPool) -> impl Future<Output = Result<Connection, Rejection>> {
    blocking::run(move || pool.get().map_err(warp::reject::custom))
}

fn session(pool: PgPool) -> BoxedFilter<(Session,)> {
    warp::any()
        .and_then(move || get_session(pool.clone()).compat())
        .boxed()
}

fn get_session(pool: PgPool) -> impl Future<Output = Result<Session, Rejection>> {
    get_connection(pool).map_ok(|connection| {
        let bytes: [u8; 32] = rand::random();
        Session {
            nonce: base64::encode(&bytes),
            connection,
            description: "Compile and share code in multiple programming languages".into(),
        }
    })
}

fn index(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .and(
            warp::post2()
                .and(warp::body::form())
                .and(connection(pool.clone()))
                .and_then(insert_paste::insert_paste)
                .or(warp::get2().and(session(pool)).and_then(index::index)),
        )
        .boxed()
}

fn display_paste(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path::param()
        .and(warp::path::end())
        .and(warp::get2())
        .and(session(pool))
        .and_then(display_paste::display_paste)
        .boxed()
}

fn options(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path("config")
        .and(warp::path::end())
        .and(warp::get2())
        .and(session(pool))
        .and_then(config::config)
        .boxed()
}

fn raw_paste(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    with_ext("txt")
        .map(warp::ext::set)
        .untuple_one()
        .and(
            warp::get2()
                .and(warp::ext::get())
                .and(connection(pool))
                .and_then(raw_paste::raw_paste)
                .with(cors()),
        )
        .boxed()
}

fn api_v0(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    let root = path!("api" / "v0").and(connection(pool));
    let language = root
        .clone()
        .and(path!("language" / String))
        .and(warp::path::end())
        .and(warp::get2())
        .and_then(api_language::api_language);
    let run = root
        .and(path!("run" / String))
        .and(warp::post2())
        .and(warp::body::form())
        .and_then(run::run);
    language.or(run).boxed()
}

fn api_v1(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    let languages = warp::path("languages")
        .and(warp::path::end())
        .and(warp::get2())
        .and(connection(pool.clone()))
        .and_then(api_v1::languages::languages);
    let pastes = warp::path("pastes")
        .and(warp::path::end())
        .and(warp::post2())
        .and(warp::body::form())
        .and(connection(pool))
        .and_then(api_v1::pastes::insert_paste);
    path!("api" / "v1")
        .and(languages.or(pastes).with(cors()))
        .boxed()
}

fn cors() -> Cors {
    warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(&[CONTENT_TYPE])
}

fn static_dir() -> BoxedFilter<(impl Reply,)> {
    warp::path("static").and(warp::fs::dir("static")).boxed()
}

fn favicon() -> BoxedFilter<(impl Reply,)> {
    warp::path("favicon.ico")
        .and(warp::path::end())
        .and(warp::fs::file("static/favicon.ico"))
        .boxed()
}

pub fn routes(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> {
    let mut headers = HeaderMap::new();
    headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    index(pool.clone())
        .or(favicon())
        .or(options(pool.clone()))
        .or(api_v0(pool.clone()))
        .or(api_v1(pool.clone()))
        .or(raw_paste(pool.clone()))
        .or(display_paste(pool.clone()))
        .or(static_dir())
        .recover(not_found(pool))
        .with(warp::reply::with::headers(headers))
        .with(warp::reply::with::default_header(
            CONTENT_SECURITY_POLICY,
            "default-src 'none'; frame-ancestors 'none'",
        ))
        .with(warp::log("pastebinrun"))
}

fn with_ext(ext: &'static str) -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
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

type NotFoundFuture =
    Compat<Pin<Box<dyn Future<Output = Result<Response<Vec<u8>>, Rejection>> + Send>>>;

fn not_found(pool: PgPool) -> impl Clone + Fn(Rejection) -> NotFoundFuture {
    move |rejection| {
        let pool = pool.clone();
        async move {
            if let Some(rejection) = rejection.find_cause::<CustomRejection>() {
                Response::builder()
                    .status(rejection.status_code())
                    .body(rejection.to_string().into_bytes())
                    .map_err(warp::reject::custom)
            } else if rejection.is_not_found() {
                let session = get_session(pool.clone()).await?;
                session
                    .render()
                    .status(StatusCode::NOT_FOUND)
                    .html(|o| templates::not_found(o, &session))
            } else {
                Err(rejection)
            }
        }
        .boxed()
        .compat()
    }
}

#[cfg(test)]
mod test {
    use super::routes;
    use crate::migration;
    use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
    use diesel::Connection;
    use once_cell::sync::Lazy;
    use scraper::{Html, Selector};
    use serde::Deserialize;
    use std::env;
    use std::str;
    use warp::filters::BoxedFilter;
    use warp::http::header::{CONTENT_LENGTH, LOCATION};
    use warp::http::StatusCode;
    use warp::reply::{Reply, Response};
    use warp::Filter;

    static ROUTES: Lazy<BoxedFilter<(Response,)>> = Lazy::new(|| {
        let pool = Pool::builder()
            .connection_customizer(Box::new(ExecuteWithinTransaction))
            .max_size(1)
            .build(ConnectionManager::new(env::var("DATABASE_URL").expect(
                "Setting DATABASE_URL environment variable required to run tests",
            )))
            .expect("Couldn't create a connection connection");
        diesel_migrations::run_pending_migrations(&pool.get().unwrap()).unwrap();
        migration::run(&pool.get().unwrap()).unwrap();
        routes(pool).map(Reply::into_response).boxed()
    });

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

    fn get_sh_id() -> String {
        let response = warp::test::request().reply(&*ROUTES);
        let document = Html::parse_document(str::from_utf8(response.body()).unwrap());
        document
            .select(&Selector::parse("#language option").unwrap())
            .find(|element| element.text().next() == Some("Sh"))
            .expect("a language called Sh to exist")
            .value()
            .attr("value")
            .expect("an ID")
            .to_string()
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Implementation<'a> {
        label: &'a str,
        #[serde(borrow)]
        wrappers: Vec<Wrapper<'a>>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Wrapper<'a> {
        identifier: &'a str,
        label: &'a str,
        is_asm: bool,
        is_formatter: bool,
    }

    #[test]
    #[cfg_attr(not(feature = "database_tests"), ignore)]
    fn test_language_api() {
        #[derive(Debug, Deserialize, PartialEq)]
        pub struct ApiLanguage<'a> {
            #[serde(borrow)]
            implementations: Vec<Implementation<'a>>,
        }
        let response = warp::test::request()
            .path(&format!("/api/v0/language/{}", get_sh_id()))
            .reply(&*ROUTES);
        assert_eq!(
            serde_json::from_slice::<ApiLanguage>(response.body()).unwrap(),
            ApiLanguage {
                implementations: vec![Implementation {
                    label: "sh",
                    wrappers: vec![Wrapper {
                        identifier: "sh",
                        label: "Run",
                        is_asm: false,
                        is_formatter: false,
                    }],
                }],
            },
        );
    }

    #[test]
    #[cfg_attr(not(feature = "database_tests"), ignore)]
    fn test_raw_pastes() {
        let body = format!("language={}&code=abc&share=share24", get_sh_id());
        let reply = warp::test::request()
            .method("POST")
            .header(CONTENT_LENGTH, body.len())
            .body(body)
            .reply(&*ROUTES);
        assert_eq!(reply.status(), StatusCode::SEE_OTHER);
        let location = reply.headers()[LOCATION].to_str().unwrap();
        assert_eq!(
            warp::test::request()
                .path(&format!("{}.txt", location))
                .reply(&*ROUTES)
                .body(),
            "abc"
        );
    }

    #[test]
    #[cfg_attr(not(feature = "sandbox_tests"), ignore)]
    fn test_sandbox() {
        #[derive(Deserialize)]
        struct LanguageIdentifier<'a> {
            identifier: &'a str,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct ApiLanguage<'a> {
            hello_world: String,
            #[serde(borrow)]
            implementations: Vec<Implementation<'a>>,
        }
        let languages = warp::test::request()
            .path("/api/v1/languages")
            .reply(&*ROUTES);
        let languages =
            serde_json::from_slice::<Vec<LanguageIdentifier>>(languages.body()).unwrap();
        for LanguageIdentifier { identifier } in languages {
            let language = warp::test::request()
                .path(&format!("/api/v0/language/{}", identifier))
                .reply(&*ROUTES);
            let ApiLanguage {
                hello_world,
                implementations,
            } = serde_json::from_slice(language.body()).unwrap();
            let wrappers = implementations
                .into_iter()
                .flat_map(|i| i.wrappers)
                .filter(|w| w.label == "Run");
            for Wrapper { identifier, .. } in wrappers {
                let body = format!("code={}&compilerOptions=&stdin=", hello_world);
                let out = warp::test::request()
                    .path(&format!("/api/v0/run/{}", identifier))
                    .method("POST")
                    .header(CONTENT_LENGTH, body.len())
                    .body(body)
                    .reply(&*ROUTES);
                let body = str::from_utf8(out.body()).unwrap();
                assert!(
                    body.contains(r#"Hello, world!\n""#),
                    "{}: {}",
                    identifier,
                    body,
                );
            }
        }
    }

    #[test]
    #[cfg_attr(not(feature = "database_tests"), ignore)]
    fn raw_cors() {
        assert_eq!(
            warp::test::request()
                .path("/a.txt")
                .method("OPTIONS")
                .header("origin", "example.com")
                .header("access-control-request-method", "GET")
                .reply(&*ROUTES)
                .status(),
            StatusCode::OK,
        );
    }

    #[test]
    #[cfg_attr(not(feature = "database_tests"), ignore)]
    fn paste_no_cors() {
        assert_eq!(
            warp::test::request()
                .path("/a")
                .method("OPTIONS")
                .header("origin", "example.com")
                .header("access-control-request-method", "GET")
                .reply(&*ROUTES)
                .status(),
            StatusCode::METHOD_NOT_ALLOWED,
        );
    }

    #[test]
    #[cfg_attr(not(feature = "database_tests"), ignore)]
    fn api_v1_cors() {
        assert_eq!(
            warp::test::request()
                .path("/api/v1/languages")
                .method("OPTIONS")
                .header("origin", "example.com")
                .header("access-control-request-method", "GET")
                .reply(&*ROUTES)
                .status(),
            StatusCode::OK,
        );
    }
}
