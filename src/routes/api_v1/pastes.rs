use crate::models::paste;
use crate::Connection;
use chrono::{DateTime, Utc};
use futures::Future;
use futures03::TryFutureExt;
use serde::Deserialize;
use tokio_executor::blocking;
use warp::Rejection;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PasteForm {
    delete_at: Option<DateTime<Utc>>,
    #[serde(default = "default_language")]
    language: String,
    code: String,
}

fn default_language() -> String {
    "plain-text".into()
}

pub fn insert_paste(
    PasteForm {
        delete_at,
        language,
        code,
    }: PasteForm,
    connection: Connection,
) -> impl Future<Item = String, Error = Rejection> {
    blocking::run(move || paste::insert(&connection, delete_at, &language, code)).compat()
}
