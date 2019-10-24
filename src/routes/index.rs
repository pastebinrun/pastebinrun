use crate::models::language::{Language, Selection};
use crate::models::session::Session;
use crate::templates::{self, RenderRucte};
use futures::Future;
use futures03::TryFutureExt;
use tokio_executor::blocking;
use warp::{Rejection, Reply};

pub fn index(session: Session) -> impl Future<Item = impl Reply, Error = Rejection> {
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
    .compat()
}
