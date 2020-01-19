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

use crate::schema::languages::dsl::*;
use crate::Connection;
use diesel::prelude::*;
use warp::Rejection;

#[derive(Queryable)]
pub struct Language {
    pub id: i32,
    pub identifier: String,
    pub name: String,
}

impl Language {
    pub fn fetch(connection: &Connection) -> Result<Vec<Language>, Rejection> {
        languages
            .select((language_id, identifier, name))
            .order((priority.asc(), name.asc()))
            .load(connection)
            .map_err(warp::reject::custom)
    }
}

pub struct Selection {
    pub languages: Vec<Language>,
    pub selected_language: Option<i32>,
}
