use crate::schema::languages::dsl::*;
use crate::PgPool;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use serde::Serialize;
use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};
use warp::http::header::CACHE_CONTROL;
use warp::{Rejection, Reply};

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
struct ApiLanguage {
    mode: Option<String>,
    mime: String,
}

pub fn api_language(
    id: i32,
    pool: &'static PgPool,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    languages
        .find(id)
        .select((highlighter_mode, mime))
        .get_result_async(pool)
        .compat()
        .then(|result| result.optional())
        .map(|paste_contents| paste_contents.ok_or_else(warp::reject::not_found))
        .map_err(warp::reject::custom)
        .flatten()
        .map(|json: ApiLanguage| {
            warp::reply::with_header(warp::reply::json(&json), CACHE_CONTROL, "max-age=14400")
        })
}
