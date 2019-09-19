use crate::models::register::Form;
use crate::templates;
use crate::templates::RenderRucte;
use warp::http::Response;
use warp::{Rejection, Reply};

pub fn register() -> Result<impl Reply, Rejection> {
    Response::builder().html(|o| templates::register(o, &Form::default()))
}
