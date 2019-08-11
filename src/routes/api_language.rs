use crate::schema::{languages, wrappers};
use crate::PgPool;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use serde::Serialize;
use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};
use warp::http::header::CACHE_CONTROL;
use warp::{Rejection, Reply};

#[derive(Queryable)]
struct QueryLanguage {
    mode: Option<String>,
    mime: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiLanguage {
    mode: Option<String>,
    mime: String,
    wrappers: Vec<Wrapper>,
}

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
struct Wrapper {
    id: i32,
    label: String,
    is_formatter: bool,
}

pub fn api_language(
    id: i32,
    pool: &'static PgPool,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    languages::table
        .find(id)
        .select((languages::highlighter_mode, languages::mime))
        .get_result_async(pool)
        .compat()
        .then(|result| result.optional())
        .map(|language| language.ok_or_else(warp::reject::not_found))
        .map_err(warp::reject::custom)
        .flatten()
        .join(
            wrappers::table
                .filter(wrappers::language_id.eq(id))
                .select((
                    wrappers::wrapper_id,
                    wrappers::label,
                    wrappers::is_formatter,
                ))
                .order(wrappers::ordering)
                .load_async(pool)
                .compat()
                .map_err(warp::reject::custom),
        )
        .map(|(language, wrappers): (QueryLanguage, _)| {
            warp::reply::with_header(
                warp::reply::json(&ApiLanguage {
                    mode: language.mode,
                    mime: language.mime,
                    wrappers,
                }),
                CACHE_CONTROL,
                "max-age=14400",
            )
        })
}
