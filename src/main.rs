#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use askama::Template;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use warp::Reply;

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn render(template: impl Template) -> impl Reply {
    warp::reply::html(template.render().unwrap())
}

fn main() {
    env_logger::init();
    warp::serve(routes::routes()).run(([127, 0, 0, 1], 8080));
}
