use crate::models::paste::{self, ExtraPasteParameters, InsertionError};
use crate::Db;
use chrono::Duration;
use chrono::Utc;
use rocket::form::{self, Form, FromFormField, Strict, ValueField};
use rocket::http::hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use rocket::http::Header;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::error::Error;

#[derive(FromForm)]
pub struct PasteForm {
    #[field(default = Expiration(None))]
    expiration: Expiration,
    #[field(default = "plaintext")]
    language: String,
    code: String,
}

struct Expiration(Option<Duration>);

impl<'r> FromFormField<'r> for Expiration {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let duration = time_parse::duration::parse_nom(field.value).map_err(Box::from)?;
        let duration =
            Duration::from_std(duration).map_err(|x| -> Box<dyn Error + Send> { Box::new(x) })?;
        Ok(Self(Some(duration)))
    }
}

pub struct CorsString(String);

impl<'r> Responder<'r, 'static> for CorsString {
    fn respond_to(self, r: &Request<'_>) -> response::Result<'static> {
        Response::build()
            .merge(self.0.respond_to(r)?)
            .header(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*"))
            .ok()
    }
}

#[post("/api/v1/pastes", data = "<form>")]
pub async fn api_insert_paste(
    db: Db,
    form: Form<Strict<PasteForm>>,
) -> Result<CorsString, InsertionError> {
    let identifier = db
        .run(move |conn| {
            paste::insert(
                conn,
                form.expiration.0.map(|expiration| Utc::now() + expiration),
                &form.language,
                &form.code,
                ExtraPasteParameters {
                    stdin: "",
                    stdout: None,
                    stderr: None,
                    exit_code: None,
                },
            )
        })
        .await?;
    Ok(CorsString(identifier))
}
