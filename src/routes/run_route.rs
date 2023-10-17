// SPDX-FileCopyrightText: 2021 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::schema::implementation_wrappers;
use crate::Db;
use diesel::prelude::*;
use once_cell::sync::Lazy;
use reqwest::Client;
use rocket::form::Form;
use rocket::response::Debug;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

static CLIENT: Lazy<Client> = Lazy::new(Client::new);
static SANDBOX_URL: Lazy<String> = Lazy::new(|| env::var("SANDBOX_URL").unwrap());

#[derive(FromForm)]
pub struct RunForm {
    code: String,
    #[field(name = "compilerOptions")]
    compiler_options: String,
    stdin: String,
}

#[derive(Serialize)]
struct Request {
    files: Files,
    stdin: String,
    code: String,
}

#[derive(Serialize)]
struct Files {
    code: File,
}

#[derive(Serialize)]
struct File {
    contents: String,
}

#[derive(Deserialize, Serialize)]
pub struct Output {
    status: Option<i32>,
    output: String,
}

#[post("/api/v0/run/<identifier>", data = "<form>")]
pub async fn run(
    db: Db,
    identifier: String,
    form: Form<RunForm>,
) -> Result<Option<Json<Output>>, Debug<Box<dyn Error + Send + Sync>>> {
    let run = || async {
        let language_code = db
            .run(|conn| {
                implementation_wrappers::table
                    .filter(implementation_wrappers::identifier.eq(identifier))
                    .select(implementation_wrappers::code)
                    .get_result(conn)
                    .optional()
            })
            .await?;
        let language_code: String = if let Some(code) = language_code {
            code
        } else {
            return Ok(None);
        };
        let RunForm {
            code,
            compiler_options,
            stdin,
        } = form.into_inner();
        let json: Output = CLIENT
            .post(SANDBOX_URL.as_str())
            .json(&Request {
                files: Files {
                    code: File { contents: code },
                },
                stdin,
                code: language_code.replace("%s", &compiler_options),
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(Some(Json(json)))
    };
    run().await.map_err(Debug)
}
