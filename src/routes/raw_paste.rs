use crate::models::paste::Paste;
use crate::schema::pastes::dsl::*;
use crate::Connection;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use tokio_executor::blocking;
use warp::Rejection;

pub fn raw_paste(
    requested_identifier: String,
    connection: Connection,
) -> impl Future<Item = String, Error = Rejection> {
    blocking::run(move || {
        Paste::delete_old(&connection)?;
        pastes
            .select(paste)
            .filter(identifier.eq(requested_identifier))
            .get_result(&connection)
            .optional()
            .map_err(warp::reject::custom)?
            .ok_or_else(warp::reject::not_found)
    })
    .compat()
}
