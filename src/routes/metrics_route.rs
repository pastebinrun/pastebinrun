// SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use prometheus::TextEncoder;
use rocket::request::Request;
use rocket::response::{self, Debug, Responder, Response};

pub struct Prometheus(String);

impl<'r> Responder<'r, 'static> for Prometheus {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Content-Type", prometheus::TEXT_FORMAT)
            .ok()
    }
}

#[get("/metrics")]
pub fn metrics() -> Result<Prometheus, Debug<prometheus::Error>> {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();
    encoder.encode_utf8(&prometheus::gather(), &mut buffer)?;
    Ok(Prometheus(buffer))
}
