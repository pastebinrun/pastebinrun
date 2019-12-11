use crate::models::db::DbErrorExt;
use crate::models::paste::Paste;
use crate::schema::pastes::dsl::*;
use crate::Connection;
use diesel::prelude::*;
use tokio_executor::blocking;
use warp::Rejection;

pub async fn raw_paste(
    requested_identifier: String,
    connection: Connection,
) -> Result<String, Rejection> {
    blocking::run(move || {
        Paste::delete_old(&connection)?;
        pastes
            .select(paste)
            .filter(identifier.eq(requested_identifier))
            .get_result(&connection)
            .optional()
            .into_rejection()?
            .ok_or_else(warp::reject::not_found)
    })
    .await
}
