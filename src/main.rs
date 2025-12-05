#![warn(clippy::pedantic)]

use std::{
    ffi::OsStr,
    fs::{File, copy, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Parser, crate_version};
use tempfile::{TempDir, tempdir_in};

const TEMPLATE: [&str; 2] = [
    include_str!("../template/src/main.rs"),
    include_str!("../template/Cargo.toml"),
];

#[derive(Debug, Parser)]
#[command(version(crate_version!()))]
struct Args {
    /// .bapple file to link
    file: PathBuf,

    /// Temporary directory to be used and discarded right after
    #[arg(short, long, default_value = ".")]
    temp_dir: PathBuf,
}

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Res<()> {
    let args = Args::parse();
    let file = args.file;
    let file_clone = file.clone();
    let file_path = file.parent().unwrap_or(Path::new("."));
    let file_name = file
        .file_stem()
        .unwrap_or(OsStr::new("output"))
        .to_string_lossy();

    let temp_dir = tempdir_in(args.temp_dir)?;

    copy_template(&temp_dir, &file_name)?;

    copy(file_clone, temp_dir.path().join("link.bapple"))?;

    run_cargo_build(&temp_dir)?;
    copy(
        temp_dir.path().join(format!("target/release/{file_name}")),
        file_path.join(&*file_name),
    )?;

    Ok(())
}

fn run_cargo_build(dir: &TempDir) -> Res<()> {
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(dir.path())
        .status()?;

    if !status.success() {
        return Err("'cargo build --release' failed".into());
    }

    Ok(())
}

fn copy_template(dir: &TempDir, file_name: &str) -> Res<()> {
    let path = dir.path().to_owned();

    let main_rs = TEMPLATE[0].replace(
        "TEMPLATE_PWD",
        &path
            .canonicalize()?
            .join("link.bapple")
            .to_string_lossy()
            .replace("\\\\?\\", "")
            .replace('\\', "/"),
    );
    let cargo_toml = TEMPLATE[1].replace("template_name", file_name);

    // Create src/main.rs
    create_dir_all(path.join("src"))?;
    let mut file = File::create_new(path.join("src/main.rs"))?;

    file.write_all(main_rs.as_bytes())?;

    // Create Cargo.toml
    let mut file = File::create_new(path.join("Cargo.toml"))?;
    file.write_all(cargo_toml.as_bytes())?;

    Ok(())
}
