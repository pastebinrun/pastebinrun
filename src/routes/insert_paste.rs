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
