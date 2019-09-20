#[macro_use]
extern crate diesel;

mod environment;
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
    warp::serve(routes::routes(pool)).run(([127, 0, 0, 1], 8080));
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[cfg(test)]
mod test {
    use diesel::prelude::*;
    use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
    use lazy_static::lazy_static;
    use std::env;

    #[derive(Debug)]
    struct ExecuteWithinTransaction;

    impl<C, E> CustomizeConnection<C, E> for ExecuteWithinTransaction
    where
        C: Connection,
    {
        fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
            conn.begin_test_transaction().unwrap();
            Ok(())
        }
    }

    lazy_static! {
        pub static ref POOL: Pool<ConnectionManager<PgConnection>> = {
            let pool = Pool::builder()
                .connection_customizer(Box::new(ExecuteWithinTransaction))
                .max_size(1)
                .build(ConnectionManager::new(env::var("DATABASE_URL").expect(
                    "Setting DATABASE_URL environment variable required to run tests",
                )))
                .expect("Couldn't create a pool connection");
            diesel_migrations::run_pending_migrations(&pool.get().unwrap()).unwrap();
            pool
        };
    }
}
