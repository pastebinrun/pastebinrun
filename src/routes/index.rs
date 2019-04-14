use crate::models::Language;
use crate::{render, Connection};
use askama::Template;
use warp::Reply;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    languages: Vec<Language>,
}

pub fn index(connection: Connection) -> impl Reply {
    let languages = Language::fetch(&connection);
    render(Index { languages })
}
