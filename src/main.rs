#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::Connection as _;
use std::env;
use std::error::Error;

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    diesel_migrations::run_pending_migrations(&PgConnection::establish(&database_url)?)?;
    warp::serve(routes::routes(&database_url)).run(([127, 0, 0, 1], 8080));
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
