// SPDX-FileCopyrightText: 2021 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Config {}

#[get("/config")]
pub fn config() -> Template {
    Template::render("config", Config {})
}
