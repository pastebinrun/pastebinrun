// pastebin.run
// Copyright (C) 2021 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
