use ructe::{Result, Ructe};
use std::fs;

fn main() -> Result<()> {
    println!(
        "cargo:rustc-env=ENTRY_FILE_PATH={}",
        fs::read_to_string("entry").expect("Please use webpack to generate JavaScript"),
    );
    Ructe::from_env()?.compile_templates("templates")
}
