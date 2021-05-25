#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    let release = std::env::var("PROFILE").unwrap() == "release";

    if release {
        let mut res = winres::WindowsResource::new();

        res.set_icon("../shared/resources/icon.ico");

        if let Err(e) = res.compile() {
            eprint!("{}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
