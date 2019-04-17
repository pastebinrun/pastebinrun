use crate::models::language::{Language, Selection};
use crate::templates::RenderRucte;
use crate::{templates, Connection};
use warp::http::Response;
use warp::{Rejection, Reply};

pub fn index(connection: Connection) -> Result<impl Reply, Rejection> {
    let languages = Language::fetch(&connection);
    Response::builder().html(|o| {
        templates::index(
            o,
            Selection {
                languages,
                selected_language: None,
            },
        )
    })
}
