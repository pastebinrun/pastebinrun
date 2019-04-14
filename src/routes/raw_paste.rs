use crate::models::Paste;
use crate::schema::pastes::dsl::*;
use crate::Connection;
use diesel::prelude::*;
use warp::Rejection;

pub fn raw_paste(requested_identifier: String, db: Connection) -> Result<String, Rejection> {
    Paste::delete_old(&db);
    pastes
        .select(paste)
        .filter(identifier.eq(requested_identifier))
        .get_result(&db)
        .optional()
        .unwrap()
        .ok_or_else(warp::reject::not_found)
}
