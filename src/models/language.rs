use crate::schema::languages::dsl::*;
use crate::Connection;
use diesel::prelude::*;
use warp::Rejection;

#[derive(Queryable)]
pub struct Language {
    pub id: i32,
    pub identifier: String,
    pub name: String,
}

impl Language {
    pub fn fetch(connection: &Connection) -> Result<Vec<Language>, Rejection> {
        languages
            .select((language_id, identifier, name))
            .order((priority.asc(), name.asc()))
            .load(connection)
            .map_err(warp::reject::custom)
    }
}

pub struct Selection {
    pub languages: Vec<Language>,
    pub selected_language: Option<i32>,
}
