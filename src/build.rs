use ructe::Ructe;
use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

fn run_command(command: &str, if_fails: &str) {
    if !Command::new("sh")
        .args(&["-c", &format!("{} 1>&2", command)])
        .stdout(Stdio::null())
        .status()
        .unwrap()
        .success()
    {
        panic!("{}", if_fails);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    for file in WalkDir::new("js") {
        println!("cargo:rerun-if-changed={}", file?.path().display());
    }
    run_command("npm install", "Installing npm modules failed");
    run_command("node_modules/.bin/webpack", "Webpack failed");
    println!(
        "cargo:rustc-env=ENTRY_FILE_PATH={}",
        fs::read_to_string("entry")?,
    );
    Ructe::from_env()
        .unwrap()
        .compile_templates("templates")
        .unwrap();
    Ok(())
}
