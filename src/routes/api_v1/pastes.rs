use crate::models::paste::{self, ExtraPasteParameters};
use crate::Connection;
use chrono::{Duration, Utc};
use futures::Future;
use futures03::TryFutureExt;
use serde::de::{Deserializer, Unexpected, Visitor};
use serde::{de, Deserialize};
use std::fmt::{self, Formatter};
use tokio_executor::blocking;
use warp::Rejection;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PasteForm {
    expiration: Option<DeserializableDuration>,
    #[serde(default = "default_language")]
    language: String,
    code: String,
}

struct DeserializableDuration(Duration);

impl<'de> Deserialize<'de> for DeserializableDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = DeserializableDuration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "an RFC 3339 duration string")
            }

            fn visit_str<E>(self, input: &str) -> Result<DeserializableDuration, E>
            where
                E: de::Error,
            {
                let parsed = time_parse::duration::parse_nom(input).map_err(|_| {
                    E::invalid_value(Unexpected::Str(input), &"an RFC 3339 duration")
                })?;
                Ok(DeserializableDuration(
                    Duration::from_std(parsed).map_err(E::custom)?,
                ))
            }
        }

        deserializer.deserialize_str(DurationVisitor)
    }
}

fn default_language() -> String {
    "plaintext".into()
}

pub fn insert_paste(
    PasteForm {
        expiration,
        language,
        code,
    }: PasteForm,
    connection: Connection,
) -> impl Future<Item = String, Error = Rejection> {
    blocking::run(move || {
        paste::insert(
            &connection,
            expiration.map(|expiration| Utc::now() + expiration.0),
            &language,
            code,
            ExtraPasteParameters {
                stdin: "".into(),
                stdout: None,
                stderr: None,
                exit_code: None,
            },
        )
    })
    .compat()
}
