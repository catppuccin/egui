use std::{fs::OpenOptions, process::Command};

fn main() {
    println!("cargo::rerun-if-changed=src/themes.rs.tera");
    let fh = OpenOptions::new()
        .write(true)
        .create(true)
        .open("src/themes.rs")
        .unwrap();
    let stat = Command::new("whiskers")
        .arg("src/themes.rs.tera")
        .stdout(fh)
        .status()
        .unwrap();
    assert!(stat.success());
}
