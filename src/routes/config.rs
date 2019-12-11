use crate::models::session::{RenderExt, Session};
use crate::templates;
use warp::{Rejection, Reply};

pub async fn config(session: Session) -> Result<impl Reply, Rejection> {
    session.render().html(|o| templates::config(o, &session))
}
