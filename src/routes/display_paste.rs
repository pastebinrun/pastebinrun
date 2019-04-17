use crate::models::language::{Language, Selection};
use crate::models::paste::{ExternPaste, Paste};
use crate::schema::{languages, pastes};
use crate::templates::RenderRucte;
use crate::{templates, Connection};
use diesel::prelude::*;
use warp::http::Response;
use warp::{Rejection, Reply};

pub fn display_paste(
    requested_identifier: String,
    db: Connection,
) -> Result<impl Reply, Rejection> {
    Paste::delete_old(&db);
    let languages = Language::fetch(&db);
    let paste = pastes::table
        .inner_join(languages::table)
        .select((
            pastes::paste,
            pastes::language_id,
            pastes::delete_at,
            languages::is_markdown,
            pastes::no_follow,
        ))
        .filter(pastes::identifier.eq(requested_identifier))
        .get_result(&db)
        .optional()
        .unwrap();
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
}
