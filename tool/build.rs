#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    use std::io::Write;

    let release = std::env::var("PROFILE").unwrap() == "release";

    let production_build = std::env::var("release-type").unwrap() == "production";

    if production_build {
        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let dest_path = std::path::Path::new(&out_dir).join("deploy.toml");

        let toml_str: String = format!("version = {}", env!("CARGO_PKG_VERSION"));

        std::fs::write(&dest_path, toml_str).unwrap()
    }

    if release {
        let mut res = winres::WindowsResource::new();

        res.set_icon("../shared/resources/icon.ico");

        match res.compile() {
            Err(e) => {
                write!(std::io::stderr(), "{}", e).unwrap();
                std::process::exit(1);
            },
            Ok(_) => {},
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
