// pastebin.run
// Copyright (C) 2022 Konrad Borowski
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
