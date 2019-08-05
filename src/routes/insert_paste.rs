use crate::schema::pastes;
use crate::Connection;
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use rand::prelude::*;
use serde::de::IgnoredAny;
use serde::Deserialize;
use warp::http::Uri;
use warp::Reply;

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

pub fn insert_paste(form: PasteForm, db: Connection) -> impl Reply {
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
        .execute(&db)
        .unwrap();
    warp::redirect(format!("/{}", cloned_identifier).parse::<Uri>().unwrap())
}
