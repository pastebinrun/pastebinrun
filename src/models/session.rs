use crate::Connection;
use warp::http::header::CONTENT_SECURITY_POLICY;
use warp::http::response::{Builder, Response};

pub struct Session {
    pub nonce: String,
    pub connection: Connection,
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
