use crate::models::session::Session;
use crate::templates::{self, RenderRucte};
use warp::{Rejection, Reply};

pub fn config(session: Session) -> Result<impl Reply, Rejection> {
    session.render().html(|o| templates::config(o, &session))
}
