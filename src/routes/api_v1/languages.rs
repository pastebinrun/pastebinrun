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

use crate::schema::languages;
use crate::Connection;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use serde::Serialize;
use tokio_executor::blocking;
use warp::{Rejection, Reply};

#[derive(Queryable, Serialize)]
struct Language {
    identifier: String,
    name: String,
}

pub fn languages(connection: Connection) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        let languages: Vec<Language> = languages::table
            .select((languages::identifier, languages::name))
            .load(&connection)
            .map_err(warp::reject::custom)?;
        Ok(warp::reply::json(&languages))
    })
    .compat()
}
