use super::WithTxt;
use crate::models::language::Language;
use crate::models::paste::Paste;
use crate::schema::{languages, pastes};
use crate::Db;
use diesel::prelude::*;
use rocket::http::uri::Origin;
use rocket::response::Debug;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct DisplayPaste {
    languages: Vec<Language>,
    description: String,
    paste: String,
    selected_id: i32,
    raw_paste_url: Origin<'static>,
}

#[get("/<identifier>", rank = 2)]
pub async fn display_paste(
    db: Db,
    identifier: String,
) -> Result<Option<Template>, Debug<diesel::result::Error>> {
    db.run(move |conn| {
        Paste::delete_old(conn)?;
        let languages = Language::fetch(conn)?;
        let paste: Option<Paste> = pastes::table
            .inner_join(languages::table.on(pastes::language_id.eq(languages::language_id)))
            .select((
                pastes::identifier,
                pastes::paste,
                pastes::language_id,
                pastes::delete_at,
                languages::identifier,
                pastes::stdin,
                pastes::exit_code,
                pastes::stdout,
                pastes::stderr,
            ))
            .filter(pastes::identifier.eq(&identifier))
            .get_result(conn)
            .optional()?;
        if let Some(paste) = paste {
            let description = generate_description(&paste.paste);
            Ok(Some(Template::render(
                "display-paste",
                &DisplayPaste {
                    languages,
                    description,
                    paste: paste.paste,
                    selected_id: paste.language_id,
                    raw_paste_url: uri!(super::raw_paste(identifier)),
                },
            )))
        } else {
            Ok(None)
        }
    })
    .await
}

fn generate_description(paste: &str) -> String {
    let mut description = paste.chars().take(239).collect();
    if description != paste {
        description += "…";
    }
    description
}
