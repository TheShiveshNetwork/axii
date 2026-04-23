use std::net::TcpStream;
use std::io::{Read, Write, BufReader};
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use common::types::{Request, Response};
use common::error::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the database server
    StartServer {
        #[arg(short, long, default_value = "127.0.0.1:6379")]
        addr: String,
        #[arg(short, long, default_value = "./data")]
        data: PathBuf,
    },
    /// Set a key-value pair
    Set {
        key: String,
        value: String,
        #[arg(short, long, default_value = "127.0.0.1:6379")]
        addr: String,
    },
    /// Get the value for a key
    Get {
        key: String,
        #[arg(short, long, default_value = "127.0.0.1:6379")]
        addr: String,
    },
    /// Delete a key
    Delete {
        key: String,
        #[arg(short, long, default_value = "127.0.0.1:6379")]
        addr: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::StartServer { addr, data } => {
            server::start_server(&addr, data)?;
        }
        Commands::Set { key, value, addr } => {
            let resp = send_request(&addr, Request::Set(key.into_bytes(), value.into_bytes()))?;
            match resp {
                Response::Ok(_) => println!("OK"),
                Response::Error(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Get { key, addr } => {
            let resp = send_request(&addr, Request::Get(key.into_bytes()))?;
            match resp {
                Response::Ok(Some(val)) => println!("{}", String::from_utf8_lossy(&val)),
                Response::Ok(None) => println!("(nil)"),
                Response::Error(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Delete { key, addr } => {
            let resp = send_request(&addr, Request::Delete(key.into_bytes()))?;
            match resp {
                Response::Ok(_) => println!("OK"),
                Response::Error(e) => eprintln!("Error: {}", e),
            }
        }
    }

    Ok(())
}

fn send_request(addr: &str, req: Request) -> Result<Response> {
    let mut stream = TcpStream::connect(addr)?;
    let bytes = bincode::serialize(&req)?;
    let len = bytes.len() as u32;
    stream.write_all(&len.to_le_bytes())?;
    stream.write_all(&bytes)?;
    stream.flush()?;

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut len_bytes = [0u8; 4];
    reader.read_exact(&mut len_bytes)?;
    let res_len = u32::from_le_bytes(len_bytes) as usize;
    let mut res_bytes = vec![0u8; res_len];
    reader.read_exact(&mut res_bytes)?;

    let response: Response = bincode::deserialize(&res_bytes)?;
    Ok(response)
}
