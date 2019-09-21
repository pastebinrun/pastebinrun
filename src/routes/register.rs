use crate::models::register::Form;
use crate::schema::{sessions, users};
use crate::templates::RenderRucte;
use crate::{environment, templates, Connection};
use cookie::{Cookie, SameSite};
use diesel::insert_into;
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use time::Duration;
use tokio_executor::blocking;
use warp::http::header::SET_COOKIE;
use warp::http::{Response, Uri};
use warp::{Rejection, Reply};

#[derive(Insertable)]
#[table_name = "users"]
pub struct User<'a> {
    nickname: &'a str,
    password: String,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct Session<'a> {
    identifier: &'a str,
    user_id: i32,
}

pub fn register() -> Result<impl Reply, Rejection> {
    Response::builder().html(|o| templates::register(o, &Form::default(), &[]))
}

pub fn post(
    mut form: Form,
    connection: Connection,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    blocking::run(move || {
        form.normalize();
        let issues = form.validate(&connection)?;
        if !issues.is_empty() {
            return Ok(Response::builder()
                .html(|o| templates::register(o, &form, &issues))?
                .into_response());
        }
        let user_id = insert_into(users::table)
            .values(User {
                nickname: &form.nickname,
                password: bcrypt::hash(form.password, 10).map_err(warp::reject::custom)?,
            })
            .returning(users::user_id)
            .get_result(&connection)
            .map_err(warp::reject::custom)?;
        let mut rng = thread_rng();
        let identifier: String = (0..22).map(|_| rng.sample(Alphanumeric)).collect();
        insert_into(sessions::table)
            .values(Session {
                identifier: &identifier,
                user_id,
            })
            .execute(&connection)
            .map_err(warp::reject::custom)?;
        let cookie = Cookie::build("sessionid", identifier)
            .secure(environment::is_production())
            .max_age(Duration::days(365))
            .http_only(true)
            .same_site(SameSite::Strict)
            .finish()
            .encoded()
            .to_string();
        Ok(
            warp::reply::with_header(warp::redirect(Uri::from_static("/")), SET_COOKIE, cookie)
                .into_response(),
        )
    })
    .compat()
}
