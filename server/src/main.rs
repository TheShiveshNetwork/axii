use std::path::PathBuf;
use common::error::Result;
use server::start_server;

fn main() -> Result<()> {
    let addr = "127.0.0.1:6379";
    let db_path = PathBuf::from("./data");
    start_server(addr, db_path)
}
