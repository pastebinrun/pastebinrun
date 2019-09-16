use crate::schema::{implementation_wrappers, implementations, languages, shared_wrappers};
use crate::PgPool;
use diesel::prelude::*;
use futures03::prelude::*;
use serde::Serialize;
use tokio_diesel::{AsyncError, AsyncRunQueryDsl, OptionalExtension};
use tokio_executor::blocking;
use warp::http::header::CACHE_CONTROL;
use warp::{Rejection, Reply};

#[derive(Queryable)]
struct Language {
    id: i32,
    mode: Option<String>,
    mime: String,
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
    identifier: String,
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
    mode: Option<String>,
    mime: String,
    shared_wrappers: Vec<Wrapper>,
    implementations: Vec<JsonImplementation>,
}

#[derive(Serialize)]
struct JsonImplementation {
    identifier: String,
    label: String,
    wrappers: Vec<Wrapper>,
}

pub async fn api_language(
    identifier: String,
    pool: &'static PgPool,
) -> Result<impl Reply, Rejection> {
    let Language { id, mode, mime } = languages::table
        .filter(languages::identifier.eq(identifier))
        .select((
            languages::language_id,
            languages::highlighter_mode,
            languages::mime,
        ))
        .get_result_async(pool)
        .await
        .optional()
        .map_err(warp::reject::custom)?
        .ok_or_else(warp::reject::not_found)?;
    let (shared_wrappers, implementations) = future::try_join(
        shared_wrappers::table
            .filter(shared_wrappers::language_id.eq(id))
            .select((
                shared_wrappers::identifier,
                shared_wrappers::label,
                shared_wrappers::is_asm,
                shared_wrappers::is_formatter,
            ))
            .order(shared_wrappers::ordering)
            .load_async(pool),
        async {
            let implementations = implementations::table
                .select((
                    implementations::implementation_id,
                    implementations::identifier,
                    implementations::label,
                ))
                .filter(implementations::language_id.eq(id))
                .load_async(pool)
                .await?;
            let (implementations, implementation_wrappers) = blocking::run(move || {
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
                    .load(&pool.get().map_err(AsyncError::Checkout)?)
                    .map_err(AsyncError::Error)?;
                Ok((implementations, implementation_wrappers))
            })
            .await?;
            Ok(implementation_wrappers
                .grouped_by(&implementations)
                .into_iter()
                .zip(implementations)
                .map(|(wrappers, implementation)| JsonImplementation {
                    identifier: implementation.identifier,
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
                .collect())
        },
    )
    .await
    .map_err(warp::reject::custom)?;
    Ok(warp::reply::with_header(
        warp::reply::json(&JsonLanguage {
            mode,
            mime,
            shared_wrappers,
            implementations,
        }),
        CACHE_CONTROL,
        "max-age=14400",
    ))
}
