use crate::models::db::DbErrorExt;
use crate::models::language::{Language, Selection};
use crate::models::paste::{ExternPaste, Paste};
use crate::models::session::{RenderExt, Session};
use crate::schema::{languages, pastes};
use crate::templates;
use diesel::prelude::*;
use tokio_executor::blocking;
use warp::{Rejection, Reply};

pub async fn display_paste(
    requested_identifier: String,
    session: Session,
) -> Result<impl Reply, Rejection> {
    blocking::run(move || {
        let connection = &session.connection;
        Paste::delete_old(connection)?;
        let languages = Language::fetch(connection)?;
        let paste: Paste = pastes::table
            .inner_join(languages::table.on(pastes::language_id.eq(languages::language_id)))
            .select((
                pastes::paste,
                pastes::language_id,
                pastes::delete_at,
                languages::identifier,
                pastes::stdin,
                pastes::exit_code,
                pastes::stdout,
                pastes::stderr,
            ))
            .filter(pastes::identifier.eq(requested_identifier))
            .get_result(connection)
            .optional()
            .into_rejection()?
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
    .await
}
