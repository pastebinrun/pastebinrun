use crate::schema::implementation_wrappers;
use crate::Connection;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use lazy_static::lazy_static;
use reqwest::r#async::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio_executor::blocking;
use warp::{Rejection, Reply};

lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref SANDBOX_URL: String = env::var("SANDBOX_URL").unwrap();
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    code: String,
    compiler_options: String,
}

#[derive(Serialize)]
struct Request {
    files: Vec<File>,
    stdin: &'static str,
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

pub fn run(
    connection: Connection,
    identifier: String,
    Form {
        code,
        compiler_options,
    }: Form,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        implementation_wrappers::table
            .filter(implementation_wrappers::identifier.eq(identifier))
            .select(implementation_wrappers::code)
            .get_result(&connection)
            .optional()
            .map_err(warp::reject::custom)?
            .ok_or_else(warp::reject::not_found)
    })
    .compat()
    .and_then(move |language_code: String| {
        CLIENT
            .post(SANDBOX_URL.as_str())
            .json(&Request {
                files: vec![File {
                    name: "code",
                    contents: code,
                }],
                stdin: "",
                code: language_code.replace("%s", &compiler_options),
            })
            .send()
            .and_then(|mut r| r.json())
            .map_err(warp::reject::custom)
    })
    .map(|json: Output| warp::reply::json(&json))
}
