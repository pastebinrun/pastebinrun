use crate::schema::languages::dsl::*;
use crate::PgConnection;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Language {
    pub id: i32,
    pub name: String,
}

impl Language {
    pub fn fetch(db: &PgConnection) -> Vec<Language> {
        languages
            .select((language_id, name))
            .order((priority.asc(), name.asc()))
            .load(db)
            .unwrap()
    }
}

pub struct Selection {
    pub languages: Vec<Language>,
    pub selected_language: Option<i32>,
}
