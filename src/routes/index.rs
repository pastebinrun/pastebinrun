use crate::models::language::{Language, Selection};
use crate::templates::RenderRucte;
use crate::{templates, Connection};
use futures::Future;
use futures03::TryFutureExt;
use tokio_executor::blocking;
use warp::http::Response;
use warp::{Rejection, Reply};

pub fn index(connection: Connection) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        let languages = Language::fetch(&connection)?;
        Response::builder().html(|o| {
            templates::index(
                o,
                Selection {
                    languages,
                    selected_language: None,
                },
            )
        })
    })
    .compat()
}
