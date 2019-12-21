mod api_language;
mod api_v1;
mod config;
mod display_paste;
mod index;
mod insert_paste;
mod raw_paste;
mod run;

use crate::models::rejection::CustomRejection;
use crate::models::session::{RenderExt, Session};
use crate::templates;
use crate::{blocking, Connection};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use futures::{Future, FutureExt};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::pin::Pin;
use warp::filters::BoxedFilter;
use warp::http::header::{
    HeaderMap, HeaderValue, CONTENT_SECURITY_POLICY, CONTENT_TYPE, REFERRER_POLICY, X_FRAME_OPTIONS,
};
use warp::http::method::Method;
use warp::http::{Response, StatusCode};
use warp::reject::Reject;
use warp::{path, Filter, Rejection, Reply};

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn connection(pool: PgPool) -> BoxedFilter<(Connection,)> {
    warp::any()
        .and_then(move || get_connection(pool.clone()))
        .boxed()
}

async fn get_connection(pool: PgPool) -> Result<Connection, Rejection> {
    blocking::run(move || {
        pool.get()
            .map_err(|e| warp::reject::custom(ConnectionError(e)))
    })
    .await
}

#[derive(Debug)]
struct ConnectionError<E>(E);

impl<E: 'static + std::fmt::Debug + Send + Sync> Reject for ConnectionError<E> {}

fn session(pool: PgPool) -> BoxedFilter<(Session,)> {
    warp::any()
        .and_then(move || get_session(pool.clone()))
        .boxed()
}

async fn get_session(pool: PgPool) -> Result<Session, Rejection> {
    let connection = get_connection(pool).await?;
    let bytes: [u8; 32] = rand::random();
    Ok(Session {
        nonce: base64::encode(&bytes),
        connection,
        description: "Compile and share code in multiple programming languages".into(),
    })
}

fn index(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .and(
            warp::post()
                .and(warp::body::form())
                .and(connection(pool.clone()))
                .and_then(insert_paste::insert_paste)
                .or(warp::get().and(session(pool)).and_then(index::index)),
        )
        .boxed()
}

fn display_paste(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path::param()
        .and(warp::path::end())
        .and(warp::get())
        .and(session(pool))
        .and_then(display_paste::display_paste)
        .boxed()
}

fn options(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path("config")
        .and(warp::path::end())
        .and(warp::get())
        .and(session(pool))
        .and_then(config::config)
        .boxed()
}

fn raw_paste(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    with_ext("txt")
        .and(warp::get())
        .and(connection(pool))
        .and_then(raw_paste::raw_paste)
        .boxed()
}

fn api_v0(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    let root = path!("api" / "v0").and(connection(pool));
    let language = root
        .clone()
        .and(path!("language" / String))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(api_language::api_language);
    let run = root
        .and(path!("run" / String))
        .and(warp::post())
        .and(warp::body::form())
        .and_then(run::run);
    language.or(run).boxed()
}

fn api_v1(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    let languages = warp::path("languages")
        .and(warp::path::end())
        .and(warp::get())
        .and(connection(pool.clone()))
        .and_then(api_v1::languages::languages);
    let pastes = warp::path("pastes")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::form())
        .and(connection(pool))
        .and_then(api_v1::pastes::insert_paste);
    path!("api" / "v1")
        .and(
            languages.or(pastes).with(
                warp::cors()
                    .allow_any_origin()
                    .allow_methods(&[Method::GET, Method::POST])
                    .allow_headers(&[CONTENT_TYPE]),
            ),
        )
        .boxed()
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
            async move {
                match (path.extension(), path.file_stem().and_then(OsStr::to_str)) {
                    (Some(received_ext), Some(file_stem)) if ext == received_ext => {
                        Ok(file_stem.to_string())
                    }
                    _ => Err(warp::reject::not_found()),
                }
            }
        })
}

fn not_found(
    pool: PgPool,
) -> impl Clone
       + Fn(Rejection) -> Pin<Box<dyn Future<Output = Result<Response<Vec<u8>>, Rejection>> + Send>>
{
    move |rejection| {
        let pool = pool.clone();
        async move {
            if let Some(rejection) = rejection.find::<CustomRejection>() {
                Ok(Response::builder()
                    .status(rejection.status_code())
                    .body(rejection.to_string().into_bytes())
                    .unwrap())
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

    async fn get_sh_id() -> String {
        let response = warp::test::request().reply(&*ROUTES);
        let document = Html::parse_document(str::from_utf8(response.await.body()).unwrap());
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

    #[tokio::test]
    #[cfg_attr(not(feature = "database_tests"), ignore)]
    async fn test_language_api() {
        #[derive(Debug, Deserialize, PartialEq)]
        pub struct ApiLanguage<'a> {
            #[serde(borrow)]
            implementations: Vec<Implementation<'a>>,
        }
        let response = warp::test::request()
            .path(&format!("/api/v0/language/{}", get_sh_id().await))
            .reply(&*ROUTES)
            .await;
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

    #[tokio::test]
    #[cfg_attr(not(feature = "database_tests"), ignore)]
    async fn test_raw_pastes() {
        let body = format!("language={}&code=abc", get_sh_id().await);
        let reply = warp::test::request()
            .method("POST")
            .header(CONTENT_LENGTH, body.len())
            .body(body)
            .reply(&*ROUTES)
            .await;
        assert_eq!(reply.status(), StatusCode::SEE_OTHER);
        let location = reply.headers()[LOCATION].to_str().unwrap();
        assert_eq!(
            warp::test::request()
                .path(&format!("{}.txt", location))
                .reply(&*ROUTES)
                .await
                .body(),
            "abc"
        );
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "sandbox_tests"), ignore)]
    async fn test_sandbox() {
        #[derive(Deserialize)]
        struct LanguageIdentifier<'a> {
            identifier: &'a str,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct ApiLanguage<'a> {
            hello_world_paste: Option<String>,
            #[serde(borrow)]
            implementations: Vec<Implementation<'a>>,
        }
        let languages = warp::test::request()
            .path("/api/v1/languages")
            .reply(&*ROUTES)
            .await;
        let languages =
            serde_json::from_slice::<Vec<LanguageIdentifier>>(languages.body()).unwrap();
        for LanguageIdentifier { identifier } in languages {
            let language = warp::test::request()
                .path(&format!("/api/v0/language/{}", identifier))
                .reply(&*ROUTES)
                .await;
            if let ApiLanguage {
                hello_world_paste: Some(hello_world_paste),
                implementations,
            } = serde_json::from_slice(language.body()).unwrap()
            {
                let code = warp::test::request()
                    .path(&format!("/{}.txt", hello_world_paste))
                    .reply(&*ROUTES)
                    .await;
                let wrappers = implementations
                    .into_iter()
                    .flat_map(|i| i.wrappers)
                    .filter(|w| w.label == "Run");
                for Wrapper { identifier, .. } in wrappers {
                    let body = format!(
                        "code={}&compilerOptions=&stdin=",
                        str::from_utf8(code.body()).unwrap()
                    );
                    let out = warp::test::request()
                        .path(&format!("/api/v0/run/{}", identifier))
                        .method("POST")
                        .header(CONTENT_LENGTH, body.len())
                        .body(body)
                        .reply(&*ROUTES)
                        .await;
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
    }
}
