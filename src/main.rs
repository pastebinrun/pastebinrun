#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod models;
mod routes;
mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::Connection as _;
use std::env;
use std::error::Error;

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    embedded_migrations::run_with_output(
        &PgConnection::establish(&database_url)?,
        &mut std::io::stdout(),
    )?;
    warp::serve(routes::routes(&database_url)).run(([127, 0, 0, 1], 8080));
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
