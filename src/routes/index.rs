use crate::models::language::{Language, Selection};
use crate::templates::RenderRucte;
use crate::{templates, PgPool};
use futures::Future;
use warp::http::Response;
use warp::{Rejection, Reply};

pub fn index(pool: &'static PgPool) -> impl Future<Item = impl Reply, Error = Rejection> {
    Language::fetch(pool)
        .map_err(warp::reject::custom)
        .and_then(|languages| {
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
}
