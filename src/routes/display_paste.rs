// pastebin.run
// Copyright (C) 2020 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::blocking;
use crate::models::db::DbErrorExt;
use crate::models::language::{Language, Selection};
use crate::models::paste::{ExternPaste, Paste};
use crate::models::session::{RenderExt, Session};
use crate::schema::{languages, pastes};
use crate::templates;
use diesel::prelude::*;
use std::borrow::Cow;
use warp::{Rejection, Reply};

pub async fn display_paste(
    requested_identifier: String,
    mut session: Session,
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
        session.description = generate_description(&paste.paste);
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

fn generate_description(paste: &str) -> Cow<'static, str> {
    let mut description = paste.chars().take(239).collect();
    if description != paste {
        description += "…";
    }
    description
}
