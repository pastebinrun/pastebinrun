use crate::models::language::{Language, Selection};
use crate::models::paste::{ExternPaste, Paste};
use crate::models::session::Session;
use crate::schema::{languages, pastes};
use crate::templates::{self, RenderRucte};
use diesel::prelude::*;
use futures::future::*;
use futures03::TryFutureExt;
use tokio_executor::blocking;
use warp::{Rejection, Reply};

pub fn display_paste(
    requested_identifier: String,
    session: Session,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        let connection = &session.connection;
        Paste::delete_old(connection)?;
        let languages = Language::fetch(connection)?;
        let paste: Paste = pastes::table
            .inner_join(languages::table)
            .select((
                pastes::paste,
                pastes::language_id,
                pastes::delete_at,
                languages::identifier,
            ))
            .filter(pastes::identifier.eq(requested_identifier))
            .get_result(connection)
            .optional()
            .map_err(warp::reject::custom)?
            .ok_or_else(warp::reject::not_found)?;
        let selected_language = Some(paste.language_id);
        session.render().html(|o| {
            templates::display_paste(
                o,
                &session,
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
