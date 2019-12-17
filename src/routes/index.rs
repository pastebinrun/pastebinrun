use crate::blocking;
use crate::models::language::{Language, Selection};
use crate::models::session::{RenderExt, Session};
use crate::templates;
use warp::{Rejection, Reply};

pub async fn index(session: Session) -> Result<impl Reply, Rejection> {
    blocking::run(move || {
        let languages = Language::fetch(&session.connection)?;
        session.render().html(|o| {
            templates::index(
                o,
                &session,
                Selection {
                    languages,
                    selected_language: None,
                },
            )
        })
    })
    .await
}
