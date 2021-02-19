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
use extension_trait::extension_trait;
use mime::TEXT_HTML_UTF_8;
use std::borrow::Cow;
use std::io;
use warp::http;
use warp::http::header::{CONTENT_SECURITY_POLICY, CONTENT_TYPE};
use warp::http::response::{Builder, Response};
use warp::reject::{Reject, Rejection};

pub struct Session {
    pub nonce: String,
    pub connection: Connection,
    pub description: Cow<'static, str>,
}

impl Session {
    pub fn render(&self) -> Builder {
        Response::builder().header(
            CONTENT_SECURITY_POLICY,
            format!(
                concat!(
                    "default-src 'none'; ",
                    "script-src 'self' 'nonce-{nonce}' 'strict-dynamic'; ",
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
        )
    }
}

#[extension_trait]
pub impl RenderExt for Builder {
    fn html<F>(self, f: F) -> Result<Response<Vec<u8>>, Rejection>
    where
        F: FnOnce(&mut Vec<u8>) -> io::Result<()>,
    {
        let mut buf = Vec::new();
        f(&mut buf).map_err(|e| warp::reject::custom(TemplateError(e)))?;
        self.header(CONTENT_TYPE, TEXT_HTML_UTF_8.as_ref())
            .body(buf)
            .map_err(|e| warp::reject::custom(RenderError(e)))
    }
}

#[derive(Debug)]
struct TemplateError(io::Error);

impl Reject for TemplateError {}

#[derive(Debug)]
struct RenderError(http::Error);

impl Reject for RenderError {}
