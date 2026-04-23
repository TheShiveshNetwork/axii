use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::PathBuf;
use common::types::{Key, Value, Record};
use common::error::Result;

pub struct SstableWriter {
    path: PathBuf,
}

impl SstableWriter {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn write(&self, entries: Vec<(Key, Option<Value>)>) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        let mut writer = BufWriter::new(file);

        let count = entries.len() as u64;
        writer.write_all(&count.to_le_bytes())?;

        for (key, value) in entries {
            let record = Record { key, value };
            let bytes = bincode::serialize(&record)?;
            let len = bytes.len() as u32;
            writer.write_all(&len.to_le_bytes())?;
            writer.write_all(&bytes)?;
        }

        writer.flush()?;
        Ok(())
    }
}

pub struct SstableReader {
    path: PathBuf,
}

impl SstableReader {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Option<Value>>> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);

        let mut count_bytes = [0u8; 8];
        reader.read_exact(&mut count_bytes)?;
        let count = u64::from_le_bytes(count_bytes);

        for _ in 0..count {
            let mut len_bytes = [0u8; 4];
            if reader.read_exact(&mut len_bytes).is_err() {
                break;
            }
            let len = u32::from_le_bytes(len_bytes) as usize;
            let mut bytes = vec![0u8; len];
            reader.read_exact(&mut bytes)?;

            let record: Record = bincode::deserialize(&bytes)?;
            if record.key == key {
                return Ok(Some(record.value));
            }
        }

        Ok(None)
    }
}
