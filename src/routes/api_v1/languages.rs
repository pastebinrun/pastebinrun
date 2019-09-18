use crate::schema::languages;
use crate::Connection;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use serde::Serialize;
use tokio_executor::blocking;
use warp::{Rejection, Reply};

#[derive(Queryable, Serialize)]
struct Language {
    identifier: String,
    name: String,
}

pub fn languages(connection: Connection) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        let languages: Vec<Language> = languages::table
            .select((languages::identifier, languages::name))
            .load(&connection)
            .map_err(warp::reject::custom)?;
        Ok(warp::reply::json(&languages))
    })
    .compat()
}
