#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

fn main() {
    env_logger::init();
    warp::serve(routes::routes()).run(([127, 0, 0, 1], 8080));
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
