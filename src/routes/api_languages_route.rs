// pastebin.run
// Copyright (C) 2021 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
