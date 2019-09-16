use crate::schema::languages::dsl::*;
use crate::PgPool;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use tokio_diesel::{AsyncError, AsyncRunQueryDsl};

#[derive(Queryable)]
pub struct Language {
    pub id: i32,
    pub identifier: String,
    pub name: String,
}

impl Language {
    pub fn fetch(pool: &'static PgPool) -> impl Future<Item = Vec<Language>, Error = AsyncError> {
        languages
            .select((language_id, identifier, name))
            .order((priority.asc(), name.asc()))
            .load_async(pool)
            .compat()
    }
}

pub struct Selection {
    pub languages: Vec<Language>,
    pub selected_language: Option<i32>,
}
