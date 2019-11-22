use crate::schema::{implementation_wrappers, implementations, languages};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Integer, Text};
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
struct Language {
    identifier: String,
    #[serde(default)]
    implementations: Vec<Implementation>,
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

pub fn run(connection: impl Connection<Backend = Pg>) -> Result<(), Box<dyn Error>> {
    let languages: Vec<Language> = serde_json::from_slice(&fs::read("languages.json")?)?;
    for Language {
        identifier: languages_identifier,
        implementations,
    } in languages
    {
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
                .execute(&connection)?;
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
                    .execute(&connection)?;
            }
        }
    }
    Ok(())
}
