// SPDX-FileCopyrightText: 2021 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::models::Cors;
use crate::schema::languages;
use crate::Db;
use diesel::prelude::*;
use rocket::response::Debug;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Language {
    identifier: String,
    name: String,
}

#[get("/api/v1/languages")]
pub async fn api_languages(
    db: Db,
) -> Result<Cors<Json<Vec<Language>>>, Debug<diesel::result::Error>> {
    let languages = db
        .run(|conn| {
            languages::table
                .select((languages::identifier, languages::name))
                .load(conn)
        })
        .await?;
    Ok(Cors(Json(languages)))
}

#[cfg(all(test, feature = "database_tests"))]
mod test {
    use rocket::http::hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::uri;

    #[test]
    fn test_cors() {
        let rocket = Client::untracked(crate::rocket()).unwrap();
        let response = rocket.get(uri!(super::api_languages)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response
                .headers()
                .get_one(ACCESS_CONTROL_ALLOW_ORIGIN.as_str()),
            Some("*"),
        );
    }
}
