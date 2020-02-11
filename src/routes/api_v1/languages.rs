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
use crate::schema::languages;
use crate::{blocking, Connection};
use diesel::prelude::*;
use serde::Serialize;
use warp::{Rejection, Reply};

#[derive(Queryable, Serialize)]
struct Language {
    identifier: String,
    name: String,
}

pub async fn languages(connection: Connection) -> Result<impl Reply, Rejection> {
    blocking::run(move || {
        let languages: Vec<Language> = languages::table
            .select((languages::identifier, languages::name))
            .load(&connection)
            .into_rejection()?;
        Ok(warp::reply::json(&languages))
    })
    .await
}
