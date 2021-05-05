#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]

use crate::utils::lcu::Endpoints;

mod ui;
mod utils;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let path = utils::process::get_lock_file_path().unwrap();

    // get lock
    let lock_file_info = utils::lock_file::parse(&path).unwrap();

    let lcu = utils::lcu::LCUClient::new(&lock_file_info.token, lock_file_info.port).unwrap();

    println!("{}", lock_file_info.token);

    let resp = lcu
        .send(Endpoints::CancelLobby, reqwest::Method::POST, "{}".into())
        .await
        .unwrap();

    println!("{:?}", resp);

    let r = lcu
        .send(Endpoints::QuickSeach, reqwest::Method::POST, r#"{"queueId": 1110}"#.into())
        .await
        .unwrap();

    println!("{:?}", r);
    Ok(())
}
