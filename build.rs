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

use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Manifest {
    #[serde(rename = "js/index.ts")]
    index: Index,
}

#[derive(Deserialize)]
struct Index {
    file: String,
    css: [String; 1],
}

fn main() -> serde_json::Result<()> {
    if env::var("PROFILE").as_deref() == Ok("release") {
        let Manifest {
            index: Index { file, css: [css] },
        } = serde_json::from_str(include_str!("dist/manifest.json"))?;
        println!("cargo:rustc-env=ENTRY_FILE_PATH={}", file);
        println!("cargo:rustc-env=CSS_PATH={}", css);
    }
    Ok(())
}
