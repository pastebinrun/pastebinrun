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

use crate::models::paste;
use crate::models::paste::ExtraPasteParameters;
use crate::Connection;
use chrono::{Duration, Utc};
use futures::Future;
use futures03::TryFutureExt;
use serde::Deserialize;
use tokio_executor::blocking;
use warp::http::header::LOCATION;
use warp::http::StatusCode;
use warp::{reply, Rejection, Reply};

#[derive(Deserialize)]
pub struct PasteForm {
    language: String,
    code: String,
    share: Share,
    #[serde(default)]
    stdin: String,
    stdout: Option<String>,
    stderr: Option<String>,
    status: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Share {
    Share,
    Share24,
}

pub fn insert_paste(
    PasteForm {
        language,
        code,
        share,
        stdin,
        stdout,
        stderr,
        status,
    }: PasteForm,
    connection: Connection,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        let delete_at = match share {
            Share::Share => None,
            Share::Share24 => Some(Utc::now() + Duration::hours(24)),
        };
        let identifier = paste::insert(
            &connection,
            delete_at,
            &language,
            code,
            ExtraPasteParameters {
                stdin,
                stdout,
                stderr,
                exit_code: status,
            },
        )?;
        Ok(reply::with_header(
            StatusCode::SEE_OTHER,
            LOCATION,
            format!("/{}", identifier),
        ))
    })
    .compat()
}
