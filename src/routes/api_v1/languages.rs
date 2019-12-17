use crate::models::db::DbErrorExt;
use crate::schema::languages;
use crate::{blocking, Connection};
use diesel::prelude::*;
use serde::Serialize;
use warp::{Rejection, Reply};

#[derive(Queryable, Serialize)]
struct Language {
    identifier: String,
    name: String,
}

pub async fn languages(connection: Connection) -> Result<impl Reply, Rejection> {
    blocking::run(move || {
        let languages: Vec<Language> = languages::table
            .select((languages::identifier, languages::name))
            .load(&connection)
            .into_rejection()?;
        Ok(warp::reply::json(&languages))
    })
    .await
}
