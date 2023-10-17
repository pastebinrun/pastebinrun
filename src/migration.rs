// SPDX-FileCopyrightText: 2020 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::schema::{implementation_wrappers, implementations, languages};
use diesel::prelude::*;
use diesel::sql_types::{Bool, Integer, Text};
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
struct JsonLanguage {
    identifier: String,
    name: String,
    #[serde(default)]
    helloworld: String,
    #[serde(default)]
    implementations: Vec<Implementation>,
}

#[derive(Insertable)]
struct Language<'a> {
    identifier: &'a str,
    name: String,
    priority: i32,
    hello_world: &'a str,
}

#[derive(Deserialize)]
struct Implementation {
    label: String,
    identifier: String,
    #[serde(default)]
    wrappers: Vec<Wrapper>,
}

#[derive(Deserialize)]
struct Wrapper {
    identifier: String,
    label: String,
    code: String,
    #[serde(default)]
    is_asm: bool,
    #[serde(default)]
    is_formatter: bool,
}

pub fn run(connection: &mut PgConnection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let languages: Vec<JsonLanguage> = serde_json::from_slice(&fs::read("languages.json")?)?;
    for JsonLanguage {
        identifier: languages_identifier,
        name,
        helloworld: hello_world,
        implementations,
    } in languages
    {
        diesel::insert_into(languages::table)
            .values(Language {
                identifier: &languages_identifier,
                name,
                priority: 10,
                hello_world: &hello_world,
            })
            .on_conflict(languages::identifier)
            .do_update()
            .set(languages::hello_world.eq(&hello_world))
            .execute(connection)?;
        for (
            i,
            Implementation {
                label,
                identifier: implementation_identifier,
                wrappers,
            },
        ) in (1..).zip(implementations)
        {
            languages::table
                .filter(languages::identifier.eq(&languages_identifier))
                .select((
                    languages::language_id,
                    label.as_sql::<Text>(),
                    implementation_identifier.as_sql::<Text>(),
                    i.as_sql::<Integer>(),
                ))
                .insert_into(implementations::table)
                .into_columns((
                    implementations::language_id,
                    implementations::label,
                    implementations::identifier,
                    implementations::ordering,
                ))
                .on_conflict((implementations::language_id, implementations::identifier))
                .do_update()
                .set((
                    implementations::label.eq(&label),
                    implementations::ordering.eq(i),
                ))
                .execute(connection)?;
            for (
                i,
                Wrapper {
                    identifier,
                    label,
                    code,
                    is_asm,
                    is_formatter,
                },
            ) in (1..).zip(wrappers)
            {
                languages::table
                    .inner_join(implementations::table)
                    .filter(languages::identifier.eq(&languages_identifier))
                    .filter(implementations::identifier.eq(&implementation_identifier))
                    .select((
                        implementations::implementation_id,
                        identifier.as_sql::<Text>(),
                        label.as_sql::<Text>(),
                        code.as_sql::<Text>(),
                        is_asm.as_sql::<Bool>(),
                        is_formatter.as_sql::<Bool>(),
                        i.as_sql::<Integer>(),
                    ))
                    .insert_into(implementation_wrappers::table)
                    .into_columns((
                        implementation_wrappers::implementation_id,
                        implementation_wrappers::identifier,
                        implementation_wrappers::label,
                        implementation_wrappers::code,
                        implementation_wrappers::is_asm,
                        implementation_wrappers::is_formatter,
                        implementation_wrappers::ordering,
                    ))
                    .on_conflict(implementation_wrappers::identifier)
                    .do_update()
                    .set((
                        implementation_wrappers::label.eq(&label),
                        implementation_wrappers::code.eq(&code),
                        implementation_wrappers::is_asm.eq(is_asm),
                        implementation_wrappers::is_formatter.eq(is_formatter),
                        implementation_wrappers::ordering.eq(i),
                    ))
                    .execute(connection)?;
            }
        }
    }
    Ok(())
}
