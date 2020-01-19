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
