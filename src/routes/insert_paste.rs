use crate::schema::pastes;
use crate::PgPool;
use chrono::{DateTime, Duration, Utc};
use futures::Future;
use futures03::TryFutureExt;
use rand::prelude::*;
use serde::de::IgnoredAny;
use serde::Deserialize;
use tokio_diesel::AsyncRunQueryDsl;
use warp::http::Uri;
use warp::{Rejection, Reply};

const CHARACTERS: &[u8] = b"23456789bcdfghjkmnpqrstvwxzBCDFGHJKLMNPQRSTVWX-";

#[derive(Deserialize)]
pub struct PasteForm {
    language: i32,
    code: String,
    autodelete: Option<IgnoredAny>,
}

#[derive(Insertable)]
#[table_name = "pastes"]
struct NewPaste {
    identifier: String,
    delete_at: Option<DateTime<Utc>>,
    language_id: i32,
    paste: String,
}

pub fn insert_paste(
    form: PasteForm,
    pool: &'static PgPool,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    let mut rng = thread_rng();
    let identifier: String = (0..10)
        .map(|_| char::from(*CHARACTERS.choose(&mut rng).expect("a random character")))
        .collect();
    let delete_at = form.autodelete.map(|_| Utc::now() + Duration::hours(24));
    let cloned_identifier = identifier.clone();
    diesel::insert_into(pastes::table)
        .values(NewPaste {
            identifier,
            delete_at,
            language_id: form.language,
            paste: form.code,
        })
        .execute_async(&pool)
        .compat()
        .map_err(warp::reject::custom)
        .and_then(move |_| {
            format!("/{}", cloned_identifier)
                .parse::<Uri>()
                .map_err(warp::reject::custom)
        })
        .map(warp::redirect)
}
