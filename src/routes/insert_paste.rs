use crate::models::paste;
use crate::Connection;
use chrono::{Duration, Utc};
use futures::Future;
use futures03::TryFutureExt;
use serde::de::IgnoredAny;
use serde::Deserialize;
use tokio_executor::blocking;
use warp::http::header::LOCATION;
use warp::http::StatusCode;
use warp::{reply, Rejection, Reply};

#[derive(Deserialize)]
pub struct PasteForm {
    language: String,
    code: String,
    autodelete: Option<IgnoredAny>,
    #[serde(default)]
    stdin: String,
    stdout: Option<String>,
    stderr: Option<String>,
    status: Option<i32>,
}

pub fn insert_paste(
    PasteForm {
        language,
        code,
        autodelete,
        stdin,
        stdout,
        stderr,
        status,
    }: PasteForm,
    connection: Connection,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        let delete_at = autodelete.map(|_| Utc::now() + Duration::hours(24));
        let identifier = paste::insert(
            &connection,
            delete_at,
            &language,
            code,
            stdin,
            stdout,
            stderr,
            status,
        )?;
        Ok(reply::with_header(
            StatusCode::SEE_OTHER,
            LOCATION,
            format!("/{}", identifier),
        ))
    })
    .compat()
}
