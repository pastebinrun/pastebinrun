use crate::models::paste;
use crate::schema::{implementation_wrappers, implementations, languages, pastes};
use crate::Connection;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Integer, Text};
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
struct JsonLanguage {
    identifier: String,
    name: String,
    helloworld: Option<String>,
    #[serde(default)]
    implementations: Vec<Implementation>,
}

#[derive(Insertable)]
struct Language<'a> {
    identifier: &'a str,
    name: String,
    priority: i32,
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

pub fn run(connection: &Connection) -> Result<(), Box<dyn Error>> {
    let languages: Vec<JsonLanguage> = serde_json::from_slice(&fs::read("languages.json")?)?;
    for JsonLanguage {
        identifier: languages_identifier,
        name,
        helloworld,
        implementations,
    } in languages
    {
        diesel::insert_into(languages::table)
            .values(Language {
                identifier: &languages_identifier,
                name,
                priority: 10,
            })
            .on_conflict(languages::identifier)
            .do_nothing()
            .execute(connection)?;
        if let Some(hello_world) = helloworld {
            let paste_id: Option<i32> = languages::table
                .filter(languages::identifier.eq(&languages_identifier))
                .select(languages::hello_world_paste_id)
                .get_result(connection)?;
            if paste_id.is_none() {
                let identifier = paste::insert(
                    connection,
                    None,
                    &languages_identifier,
                    hello_world,
                    "".into(),
                    None,
                    None,
                    None,
                )
                .unwrap();
                diesel::update(languages::table)
                    .set(
                        languages::hello_world_paste_id.eq(pastes::table
                            .select(pastes::paste_id)
                            .filter(pastes::identifier.eq(identifier))
                            .single_value()),
                    )
                    .filter(languages::identifier.eq(&languages_identifier))
                    .execute(connection)?;
            }
        }
        for Implementation {
            label,
            identifier: implementation_identifier,
            wrappers,
        } in implementations
        {
            languages::table
                .filter(languages::identifier.eq(&languages_identifier))
                .select((
                    languages::language_id,
                    label.as_sql::<Text>(),
                    implementation_identifier.as_sql::<Text>(),
                ))
                .insert_into(implementations::table)
                .into_columns((
                    implementations::language_id,
                    implementations::label,
                    implementations::identifier,
                ))
                .on_conflict((implementations::language_id, implementations::identifier))
                .do_update()
                .set(implementations::label.eq(&label))
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
