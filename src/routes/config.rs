use crate::templates::{self, RenderRucte};
use warp::http::Response;
use warp::{Rejection, Reply};

pub fn config() -> Result<impl Reply, Rejection> {
    Response::builder().html(|o| templates::config(o))
}
