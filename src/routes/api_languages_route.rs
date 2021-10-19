use crate::schema::languages;
use crate::Db;
use diesel::prelude::*;
use rocket::response::Debug;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Language {
    identifier: String,
    name: String,
}

#[get("/api/v1/languages")]
pub async fn api_languages(db: Db) -> Result<Json<Vec<Language>>, Debug<diesel::result::Error>> {
    let languages = db
        .run(|conn| {
            languages::table
                .select((languages::identifier, languages::name))
                .load(conn)
        })
        .await?;
    Ok(Json(languages))
}
