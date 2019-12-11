use crate::models::db::DbErrorExt;
use crate::schema::implementation_wrappers;
use crate::Connection;
use diesel::prelude::*;
use futures03::TryFutureExt;
use lazy_static::lazy_static;
use reqwest::r#async::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio_executor::blocking;
use warp::reject::Reject;
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
