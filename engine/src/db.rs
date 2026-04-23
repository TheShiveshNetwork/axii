use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use parking_lot::{Mutex, RwLock};
use crate::memtable::Memtable;
use crate::wal::WalManager;
use crate::sstable::{SstableWriter, SstableReader};
use common::types::{Key, Value, Operation};
use common::error::Result;

const FLUSH_THRESHOLD: usize = 1024; // 1KB for demo

pub struct Database {
    path: PathBuf,
    memtable: Arc<Memtable>,
    wal: Arc<Mutex<WalManager>>,
    sstables: Arc<RwLock<Vec<SstableReader>>>,
}

impl Database {
    pub fn open(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        
        let memtable = Arc::new(Memtable::new());
        let wal_path = path.join("wal.log");
        let wal = WalManager::new(wal_path)?;
        
        wal.recover(&memtable)?;
        
        // Load existing sstables
        let mut sstables = Vec::new();
        let entries = fs::read_dir(&path)?;
        let mut sstable_files: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sst"))
            .collect();
        
        // Sort sstables by name (timestamp) to read newest first
        sstable_files.sort_by_key(|e| e.file_name());
        sstable_files.reverse();

        for entry in sstable_files {
            sstables.push(SstableReader::new(entry.path()));
        }
        
        Ok(Self {
            path,
            memtable,
            wal: Arc::new(Mutex::new(wal)),
            sstables: Arc::new(RwLock::new(sstables)),
        })
    }

    pub fn set(&self, key: Key, value: Value) -> Result<()> {
        let op = Operation::Set(key.clone(), value.clone());
        self.wal.lock().append(&op)?;
        self.memtable.set(key, value);
        
        if self.memtable.size() >= FLUSH_THRESHOLD {
            self.flush()?;
        }
        
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Value>> {
        // 1. Search memtable
        if let Some(result) = self.memtable.get(key) {
            return Ok(result);
        }
        
        // 2. Search sstables (newest first)
        let sstables = self.sstables.read();
        for sstable in sstables.iter() {
            if let Some(result) = sstable.get(key)? {
                return Ok(result);
            }
        }
        
        Ok(None)
    }

    pub fn delete(&self, key: Key) -> Result<()> {
        let op = Operation::Delete(key.clone());
        self.wal.lock().append(&op)?;
        self.memtable.delete(key);
        
        if self.memtable.size() >= FLUSH_THRESHOLD {
            self.flush()?;
        }
        
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis();
        let sstable_path = self.path.join(format!("{}.sst", timestamp));
        
        let entries = self.memtable.entries();
        if entries.is_empty() {
            return Ok(());
        }

        let writer = SstableWriter::new(sstable_path.clone());
        writer.write(entries)?;
        
        // Rotate WAL
        let mut wal = self.wal.lock();
        let wal_path = self.path.join("wal.log");
        let old_wal_path = self.path.join(format!("wal.{}.old", timestamp));
        fs::rename(&wal_path, &old_wal_path)?;
        *wal = WalManager::new(wal_path)?;
        
        // Clear memtable and add sstable to list
        self.memtable.clear();
        let mut sstables = self.sstables.write();
        sstables.insert(0, SstableReader::new(sstable_path));
        
        Ok(())
    }
}
