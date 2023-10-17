// SPDX-FileCopyrightText: 2021 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod api_insert_paste_route;
mod api_language_route;
mod api_languages_route;
mod config_route;
mod display_paste_route;
mod index_route;
mod insert_paste_route;
mod metrics_route;
mod raw_paste_route;
mod run_route;

pub use api_insert_paste_route::*;
pub use api_language_route::*;
pub use api_languages_route::*;
pub use config_route::*;
pub use display_paste_route::*;
pub use index_route::*;
pub use insert_paste_route::*;
pub use metrics_route::*;
pub use raw_paste_route::*;
pub use run_route::*;
