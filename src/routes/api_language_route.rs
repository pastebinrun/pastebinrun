// pastebin.run
// Copyright (C) 2020-2021 Konrad Borowski
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

use crate::schema::{implementation_wrappers, implementations, languages};
use crate::Db;
use diesel::prelude::*;
use rocket::response::Debug;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Queryable)]
struct Language {
    id: i32,
    hello_world: Option<String>,
}

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
struct Wrapper {
    identifier: String,
    label: String,
    is_asm: bool,
    is_formatter: bool,
}

#[derive(Identifiable, Queryable)]
struct Implementation {
    id: i32,
    label: String,
}

#[derive(Associations, Identifiable, Queryable)]
#[diesel(belongs_to(Implementation))]
struct ImplementationWrapper {
    id: i32,
    implementation_id: i32,
    identifier: String,
    label: String,
    is_asm: bool,
    is_formatter: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonLanguage {
    hello_world: Option<String>,
    implementations: Vec<JsonImplementation>,
}

#[derive(Serialize)]
struct JsonImplementation {
    label: String,
    wrappers: Vec<Wrapper>,
}

#[get("/api/v0/language/<identifier>")]
pub async fn api_language(
    db: Db,
    identifier: String,
) -> Result<Option<Json<JsonLanguage>>, Debug<diesel::result::Error>> {
    db.run(|conn| {
        let language: Option<Language> = languages::table
            .filter(languages::identifier.eq(identifier))
            .select((languages::language_id, languages::hello_world))
            .get_result(conn)
            .optional()?;
        Ok(if let Some(language) = language {
            let implementations: Vec<Implementation> = implementations::table
                .select((implementations::implementation_id, implementations::label))
                .filter(implementations::language_id.eq(language.id))
                .order(implementations::ordering)
                .load(conn)?;
            let implementation_wrappers = ImplementationWrapper::belonging_to(&implementations)
                .select((
                    implementation_wrappers::implementation_wrapper_id,
                    implementation_wrappers::implementation_id,
                    implementation_wrappers::identifier,
                    implementation_wrappers::label,
                    implementation_wrappers::is_asm,
                    implementation_wrappers::is_formatter,
                ))
                .order(implementation_wrappers::ordering)
                .load(conn)?;
            let implementations = implementation_wrappers
                .grouped_by(&implementations)
                .into_iter()
                .zip(implementations)
                .map(|(wrappers, implementation)| JsonImplementation {
                    label: implementation.label,
                    wrappers: wrappers
                        .into_iter()
                        .map(
                            |ImplementationWrapper {
                                 identifier,
                                 label,
                                 is_asm,
                                 is_formatter,
                                 ..
                             }| {
                                Wrapper {
                                    identifier,
                                    label,
                                    is_asm,
                                    is_formatter,
                                }
                            },
                        )
                        .collect(),
                })
                .collect();
            Some(Json(JsonLanguage {
                implementations,
                hello_world: language.hello_world,
            }))
        } else {
            None
        })
    })
    .await
}
