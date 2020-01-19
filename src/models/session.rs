// pastebin.run
// Copyright (C) 2020 Konrad Borowski
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

use crate::Connection;
use std::borrow::Cow;
use warp::http::header::CONTENT_SECURITY_POLICY;
use warp::http::response::{Builder, Response};

pub struct Session {
    pub nonce: String,
    pub connection: Connection,
    pub description: Cow<'static, str>,
}

impl Session {
    pub fn render(&self) -> Builder {
        let mut builder = Response::builder();
        builder.header(
            CONTENT_SECURITY_POLICY,
            format!(
                concat!(
                    "default-src 'none'; ",
                    "script-src 'self' https://cdnjs.cloudflare.com 'nonce-{nonce}' 'strict-dynamic'; ",
                    "style-src 'self' 'unsafe-inline'; ",
                    "connect-src 'self'; ",
                    "img-src https: data:; ",
                    "object-src 'none'; ",
                    "base-uri 'none'; ",
                    "form-action 'self'; ",
                    "frame-ancestors 'none'",
                ),
                nonce = self.nonce,
            ),
        );
        builder
    }
}
