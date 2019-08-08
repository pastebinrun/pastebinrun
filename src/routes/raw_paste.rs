use crate::models::paste::Paste;
use crate::schema::pastes::dsl::*;
use crate::PgPool;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};
use warp::Rejection;

pub fn raw_paste(
    requested_identifier: String,
    pool: &'static PgPool,
) -> impl Future<Item = String, Error = Rejection> {
    Paste::delete_old(pool)
        .and_then(move |()| {
            pastes
                .select(paste)
                .filter(identifier.eq(requested_identifier))
                .get_result_async(pool)
                .compat()
                .then(|result| result.optional())
                .map(|paste_contents| paste_contents.ok_or_else(warp::reject::not_found))
        })
        .map_err(warp::reject::custom)
        .flatten()
}
