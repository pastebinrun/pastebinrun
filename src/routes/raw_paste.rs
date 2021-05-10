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

use crate::models::db::DbErrorExt;
use crate::models::paste::Paste;
use crate::schema::pastes::dsl::*;
use crate::{blocking, Connection};
use diesel::prelude::*;
use warp::http::StatusCode;
use warp::reply::WithStatus;
use warp::Rejection;

pub async fn raw_paste(
    requested_identifier: String,
    connection: Connection,
) -> Result<WithStatus<String>, Rejection> {
    blocking::run(move || {
        Paste::delete_old(&connection)?;
        pastes
            .select(paste)
            .filter(identifier.eq(requested_identifier))
            .get_result(&connection)
            .optional()
            .into_rejection()
    })
    .await
    .map(|reply| match reply {
        Some(reply) => warp::reply::with_status(reply, StatusCode::OK),
        None => warp::reply::with_status("404 Not Found".into(), StatusCode::NOT_FOUND),
    })
}
