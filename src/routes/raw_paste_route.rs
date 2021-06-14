use crate::models::paste::Paste;
use crate::schema::pastes;
use crate::Db;
use diesel::prelude::*;
use rocket::http::impl_from_uri_param_identity;
use rocket::http::uri::fmt::{Formatter, FromUriParam, Path, UriDisplay};
use rocket::request::FromParam;
use rocket::response::Debug;
use std::fmt;

pub struct WithTxt(String);

impl<'a> FromParam<'a> for WithTxt {
    type Error = &'a str;

    fn from_param(param: &str) -> Result<Self, &str> {
        if let Some(param) = param.strip_suffix(".txt") {
            Ok(WithTxt(String::from_param(param)?))
        } else {
            Err(param)
        }
    }
}

impl UriDisplay<Path> for WithTxt {
    fn fmt(&self, f: &mut Formatter<Path>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_raw(".txt")
    }
}

impl_from_uri_param_identity!([Path] WithTxt);

impl FromUriParam<Path, String> for WithTxt {
    type Target = WithTxt;

    fn from_uri_param(param: String) -> WithTxt {
        WithTxt(param)
    }
}

#[get("/<identifier>")]
pub async fn raw_paste(
    db: Db,
    identifier: WithTxt,
) -> Result<Option<String>, Debug<diesel::result::Error>> {
    db.run(move |conn| {
        Paste::delete_old(conn)?;
        Ok(pastes::table
            .select(pastes::paste)
            .filter(pastes::identifier.eq(&identifier.0))
            .get_result(conn)
            .optional()?)
    })
    .await
}
