// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

fn main() -> serde_json::Result<()> {
    #[cfg(not(debug_assertions))]
    {
        use serde::Deserialize;

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

        let Manifest {
            index: Index { file, css: [css] },
        } = serde_json::from_str(include_str!("../dist/manifest.json"))?;
        println!("cargo:rustc-env=ENTRY_FILE_PATH={}", file);
        println!("cargo:rustc-env=CSS_PATH={}", css);
    }
    Ok(())
}
