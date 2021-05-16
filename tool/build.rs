#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    use std::io::Write;

    let release = std::env::var("PROFILE").unwrap() == "release";

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
