use crate::models::language::{Language, Selection};
use crate::models::paste::{ExternPaste, Paste};
use crate::schema::{languages, pastes};
use crate::templates::RenderRucte;
use crate::{templates, Connection};
use diesel::prelude::*;
use futures::future::*;
use futures03::TryFutureExt;
use tokio_executor::blocking;
use warp::http::Response;
use warp::{Rejection, Reply};

pub fn display_paste(
    requested_identifier: String,
    connection: Connection,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        Paste::delete_old(&connection)?;
        let languages = Language::fetch(&connection)?;
        let paste: Paste = pastes::table
            .inner_join(languages::table)
            .select((
                pastes::paste,
                pastes::language_id,
                pastes::delete_at,
                languages::is_markdown,
            ))
            .filter(pastes::identifier.eq(requested_identifier))
            .get_result(&connection)
            .optional()
            .map_err(warp::reject::custom)?
            .ok_or_else(warp::reject::not_found)?;
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
    .compat()
}
