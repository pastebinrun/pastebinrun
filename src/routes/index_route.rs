// SPDX-FileCopyrightText: 2021 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::models::language::Language;
use crate::Db;
use rocket::response::Debug;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Index {
    languages: Vec<Language>,
}

#[get("/")]
pub async fn index(db: Db) -> Result<Template, Debug<diesel::result::Error>> {
    let languages = db.run(Language::fetch).await?;
    Ok(Template::render("index", &Index { languages }))
}
