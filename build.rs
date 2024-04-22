use std::{
    fs,
    path::Path,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    println!("cargo::rerun-if-changed=src/themes.rs.tera");

    // src/themes.rs.tera is excluded from the distributed crate so whiskers is
    // not run if the crate is used as a dependency.
    if !Path::new("src/themes.rs.tera").exists() {
        return ExitCode::SUCCESS;
    }

    let outfile = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("src/themes.rs")
        .unwrap();

    let Ok(stat) = Command::new("whiskers")
        .arg("src/themes.rs.tera")
        .stdout(outfile)
        .status()
    else {
        println!("cargo::warning=Failed to run whiskers (https://github.com/catppuccin/toolbox/tree/main/whiskers). Is it installed?");
        return ExitCode::SUCCESS;
    };

    if !stat.success() {
        println!("cargo::warning=whiskers exited nonzero");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
