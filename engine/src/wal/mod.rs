use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::PathBuf;
use common::error::Result;
use common::types::Operation;
use crate::memtable::Memtable;

pub struct WalManager {
    path: PathBuf,
    writer: BufWriter<File>,
}

impl WalManager {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)?;
        
        Ok(Self {
            path,
            writer: BufWriter::new(file),
        })
    }

    pub fn append(&mut self, op: &Operation) -> Result<()> {
        let bytes = bincode::serialize(op)?;
        let len = bytes.len() as u32;
        self.writer.write_all(&len.to_le_bytes())?;
        self.writer.write_all(&bytes)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn recover(&self, memtable: &Memtable) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);

        loop {
            let mut len_bytes = [0u8; 4];
            if reader.read_exact(&mut len_bytes).is_err() {
                break;
            }
            let len = u32::from_le_bytes(len_bytes) as usize;
            let mut bytes = vec![0u8; len];
            reader.read_exact(&mut bytes)?;

            let op: Operation = bincode::deserialize(&bytes)?;
            match op {
                Operation::Set(k, v) => memtable.set(k, v),
                Operation::Delete(k) => memtable.delete(k),
            }
        }

        Ok(())
    }
}
