// pastebin.run
// Copyright (C) 2020 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
extern crate diesel;

mod migration;
mod models;
mod routes;
mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;
use std::error::Error;

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    let pool = Pool::new(ConnectionManager::new(database_url))
        .expect("Couldn't create a connection connection");
    diesel_migrations::run_pending_migrations(&pool.get()?)?;
    migration::run(&pool.get()?)?;
    warp::serve(routes::routes(pool)).run(([0, 0, 0, 0], 8080));
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
