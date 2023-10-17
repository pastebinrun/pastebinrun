// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use rocket::http::hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use rocket::http::Header;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

pub struct Cors<R>(pub R);

impl<'r, 'o, R> Responder<'r, 'o> for Cors<R>
where
    'o: 'r,
    R: Responder<'r, 'o>,
{
    fn respond_to(self, r: &'r Request<'_>) -> response::Result<'o> {
        Response::build()
            .merge(self.0.respond_to(r)?)
            .header(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*"))
            .ok()
    }
}
