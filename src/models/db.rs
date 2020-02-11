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

use extension_trait::extension_trait;
use warp::reject::{Reject, Rejection};

#[derive(Debug)]
struct DbError(diesel::result::Error);

impl Reject for DbError {}

#[extension_trait(pub)]
impl<T> DbErrorExt for Result<T, diesel::result::Error> {
    type Error = T;
    fn into_rejection(self) -> Result<Self::Error, Rejection> {
        self.map_err(|e| warp::reject::custom(DbError(e)))
    }
}
