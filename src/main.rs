#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use std::error::Error;

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    let pool = &*Box::leak(Box::new(
        Pool::new(ConnectionManager::new(database_url)).expect("Couldn't create a connection pool"),
    ));
    diesel_migrations::run_pending_migrations(&pool.get()?)?;
    warp::serve(routes::routes(pool)).run(([127, 0, 0, 1], 8080));
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
