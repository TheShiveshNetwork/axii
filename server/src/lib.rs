use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader};
use std::sync::Arc;
use std::path::PathBuf;
use engine::db::Database;
use common::types::{Request, Response};
use common::error::Result;

pub fn start_server(addr: &str, db_path: PathBuf) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on {}", addr);

    let db = Arc::new(Database::open(db_path)?);

    for stream in listener.incoming() {
        let stream = stream?;
        let db = Arc::clone(&db);
        std::thread::spawn(move || {
            if let Err(e) = handle_client(stream, db) {
                eprintln!("Error handling client: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, db: Arc<Database>) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    
    loop {
        let mut len_bytes = [0u8; 4];
        if reader.read_exact(&mut len_bytes).is_err() {
            break; // Connection closed
        }
        let len = u32::from_le_bytes(len_bytes) as usize;
        let mut bytes = vec![0u8; len];
        reader.read_exact(&mut bytes)?;

        let request: Request = bincode::deserialize(&bytes)?;
        let response = match request {
            Request::Get(key) => {
                match db.get(&key) {
                    Ok(val) => Response::Ok(val),
                    Err(e) => Response::Error(e.to_string()),
                }
            }
            Request::Set(key, value) => {
                match db.set(key, value) {
                    Ok(_) => Response::Ok(None),
                    Err(e) => Response::Error(e.to_string()),
                }
            }
            Request::Delete(key) => {
                match db.delete(key) {
                    Ok(_) => Response::Ok(None),
                    Err(e) => Response::Error(e.to_string()),
                }
            }
        };

        let res_bytes = bincode::serialize(&response)?;
        let res_len = res_bytes.len() as u32;
        stream.write_all(&res_len.to_le_bytes())?;
        stream.write_all(&res_bytes)?;
        stream.flush()?;
    }

    Ok(())
}
