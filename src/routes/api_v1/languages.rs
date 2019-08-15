use crate::schema::languages;
use crate::PgPool;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use serde::Serialize;
use tokio_diesel::AsyncRunQueryDsl;
use warp::{Rejection, Reply};

#[derive(Queryable, Serialize)]
struct Language {
    identifier: String,
    name: String,
}

pub fn languages(pool: &'static PgPool) -> impl Future<Item = impl Reply, Error = Rejection> {
    languages::table
        .select((languages::identifier, languages::name))
        .load_async(pool)
        .compat()
        .map(|languages: Vec<Language>| warp::reply::json(&languages))
        .map_err(warp::reject::custom)
}
