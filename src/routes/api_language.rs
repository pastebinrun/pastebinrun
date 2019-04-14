use crate::schema::languages::dsl::*;
use crate::Connection;
use diesel::prelude::*;
use serde::Serialize;
use warp::http::header::CACHE_CONTROL;
use warp::{Rejection, Reply};

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
struct ApiLanguage {
    mode: Option<String>,
    mime: String,
}

pub fn api_language(id: i32, db: Connection) -> Result<impl Reply, Rejection> {
    languages
        .find(id)
        .select((highlighter_mode, mime))
        .get_result(&db)
        .optional()
        .unwrap()
        .ok_or_else(warp::reject::not_found)
        .map(|json: ApiLanguage| {
            warp::reply::with_header(warp::reply::json(&json), CACHE_CONTROL, "max-age=14400")
        })
}
