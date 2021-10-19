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

mod api_insert_paste_route;
mod api_language_route;
mod api_languages_route;
mod display_paste_route;
mod index_route;
mod insert_paste_route;
mod raw_paste_route;
mod run_route;

pub use api_insert_paste_route::*;
pub use api_language_route::*;
pub use api_languages_route::*;
pub use display_paste_route::*;
pub use index_route::*;
pub use insert_paste_route::*;
pub use raw_paste_route::*;
pub use run_route::*;
