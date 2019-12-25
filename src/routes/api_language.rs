use crate::schema::{implementation_wrappers, implementations, languages, pastes};
use crate::Connection;
use diesel::prelude::*;
use futures::Future;
use futures03::prelude::*;
use serde::Serialize;
use tokio_executor::blocking;
use warp::http::header::CACHE_CONTROL;
use warp::{Rejection, Reply};

#[derive(Queryable)]
struct Language {
    id: i32,
    paste_identifier: Option<String>,
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
#[belongs_to(Implementation)]
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
struct JsonLanguage {
    hello_world_paste: Option<String>,
    implementations: Vec<JsonImplementation>,
}

#[derive(Serialize)]
struct JsonImplementation {
    label: String,
    wrappers: Vec<Wrapper>,
}

pub fn api_language(
    connection: Connection,
    identifier: String,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        let language: Language = languages::table
            .filter(languages::identifier.eq(identifier))
            .select((
                languages::language_id,
                pastes::table
                    .select(pastes::identifier)
                    .filter(languages::hello_world_paste_id.eq(pastes::paste_id.nullable()))
                    .single_value(),
            ))
            .get_result(&connection)
            .optional()
            .map_err(warp::reject::custom)?
            .ok_or_else(warp::reject::not_found)?;
        let implementations = implementations::table
            .select((implementations::implementation_id, implementations::label))
            .filter(implementations::language_id.eq(language.id))
            .order(implementations::ordering)
            .load(&connection)
            .map_err(warp::reject::custom)?;
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
            .load(&connection)
            .map_err(warp::reject::custom)?;
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
        Ok(warp::reply::with_header(
            warp::reply::json(&JsonLanguage {
                implementations,
                hello_world_paste: language.paste_identifier,
            }),
            CACHE_CONTROL,
            "max-age=14400",
        ))
    })
    .compat()
}
