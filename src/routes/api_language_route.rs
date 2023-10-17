// SPDX-FileCopyrightText: 2020 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
