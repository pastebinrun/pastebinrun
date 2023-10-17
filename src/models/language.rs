// SPDX-FileCopyrightText: 2020 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::schema::languages::dsl::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Language {
    pub id: i32,
    pub identifier: String,
    pub name: String,
}

impl Language {
    pub fn fetch(connection: &mut PgConnection) -> Result<Vec<Language>, diesel::result::Error> {
        languages
            .select((language_id, identifier, name))
            .order((priority.asc(), name.asc()))
            .load(connection)
    }
}
