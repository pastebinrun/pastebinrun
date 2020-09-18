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
use crate::schema::implementation_wrappers;
use crate::{blocking, Connection};
use diesel::prelude::*;
use futures_util::future::TryFutureExt;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use warp::reject::Reject;
use warp::{Rejection, Reply};

static CLIENT: Lazy<Client> = Lazy::new(Client::new);
static SANDBOX_URL: Lazy<String> = Lazy::new(|| env::var("SANDBOX_URL").unwrap());

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    code: String,
    compiler_options: String,
    stdin: String,
}

#[derive(Serialize)]
struct Request {
    files: Vec<File>,
    stdin: String,
    code: String,
}

#[derive(Serialize)]
struct File {
    name: &'static str,
    contents: String,
}

#[derive(Deserialize, Serialize)]
struct Output {
    status: Option<i32>,
    stdout: String,
    stderr: String,
}

pub async fn run(
    connection: Connection,
    identifier: String,
    Form {
        code,
        compiler_options,
        stdin,
    }: Form,
) -> Result<impl Reply, Rejection> {
    let language_code: String = blocking::run(move || {
        implementation_wrappers::table
            .filter(implementation_wrappers::identifier.eq(identifier))
            .select(implementation_wrappers::code)
            .get_result(&connection)
            .optional()
            .into_rejection()?
            .ok_or_else(warp::reject::not_found)
    })
    .await?;
    let json: Output = CLIENT
        .post(SANDBOX_URL.as_str())
        .json(&Request {
            files: vec![File {
                name: "code",
                contents: code,
            }],
            stdin,
            code: language_code.replace("%s", &compiler_options),
        })
        .send()
        .and_then(|r| r.json())
        .map_err(|e| warp::reject::custom(RemoteServerError(e)))
        .await?;
    Ok(warp::reply::json(&json))
}

#[derive(Debug)]
struct RemoteServerError(reqwest::Error);

impl Reject for RemoteServerError {}
