use crate::schema::{languages, pastes};
use crate::Connection;
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use rand::prelude::*;
use serde::de::IgnoredAny;
use serde::Deserialize;
use tokio_executor::blocking;
use warp::http::Uri;
use warp::{Rejection, Reply};

const CHARACTERS: &[u8] = b"23456789bcdfghjkmnpqrstvwxzBCDFGHJKLMNPQRSTVWX-";

#[derive(Deserialize)]
pub struct PasteForm {
    language: String,
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
    connection: Connection,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        let mut rng = thread_rng();
        let identifier: String = (0..10)
            .map(|_| char::from(*CHARACTERS.choose(&mut rng).expect("a random character")))
            .collect();
        let cloned_identifier = identifier.clone();
        let PasteForm {
            language,
            code,
            autodelete,
        } = form;
        let delete_at = autodelete.map(|_| Utc::now() + Duration::hours(24));
        let language_id = languages::table
            .select(languages::language_id)
            .filter(languages::identifier.eq(language))
            .get_result(&connection)
            .map_err(warp::reject::custom)?;
        diesel::insert_into(pastes::table)
            .values(NewPaste {
                identifier,
                delete_at,
                language_id,
                paste: code,
            })
            .execute(&connection)
            .map_err(warp::reject::custom)?;

        Ok(warp::redirect(
            format!("/{}", cloned_identifier)
                .parse::<Uri>()
                .map_err(warp::reject::custom)?,
        ))
    })
    .compat()
}
