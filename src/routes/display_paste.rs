use crate::models::language::{Language, Selection};
use crate::models::paste::{ExternPaste, Paste};
use crate::schema::{languages, pastes};
use crate::templates::RenderRucte;
use crate::{templates, PgPool};
use diesel::prelude::*;
use futures::future::*;
use futures03::TryFutureExt;
use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};
use warp::http::Response;
use warp::{Rejection, Reply};

pub fn display_paste(
    requested_identifier: String,
    pool: &'static PgPool,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    Paste::delete_old(pool)
        .and_then(move |()| Language::fetch(pool))
        .and_then(move |languages| {
            pastes::table
                .inner_join(languages::table)
                .select((
                    pastes::paste,
                    pastes::language_id,
                    pastes::delete_at,
                    languages::is_markdown,
                    pastes::no_follow,
                ))
                .filter(pastes::identifier.eq(requested_identifier))
                .get_result_async(pool)
                .compat()
                .then(|result| result.optional())
                .map(|paste| {
                    paste
                        .ok_or_else(warp::reject::not_found)
                        .and_then(|paste: Paste| {
                            let selected_language = Some(paste.language_id);
                            Response::builder().html(|o| {
                                templates::display_paste(
                                    o,
                                    ExternPaste::from_paste(paste),
                                    Selection {
                                        languages,
                                        selected_language,
                                    },
                                )
                            })
                        })
                })
        })
        .map_err(warp::reject::custom)
        .flatten()
}
