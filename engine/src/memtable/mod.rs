use std::collections::BTreeMap;
use parking_lot::RwLock;
use common::types::{Key, Value};

pub struct Memtable {
    map: RwLock<BTreeMap<Key, Option<Value>>>,
    size: RwLock<usize>,
}

impl Memtable {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(BTreeMap::new()),
            size: RwLock::new(0),
        }
    }

    pub fn set(&self, key: Key, value: Value) {
        let mut map = self.map.write();
        let mut size = self.size.write();
        
        let key_len = key.len();
        let val_len = value.len();
        
        if let Some(old) = map.insert(key, Some(value)) {
            if let Some(old_val) = old {
                *size = (*size + val_len).saturating_sub(old_val.len());
            } else {
                *size += val_len;
            }
        } else {
            *size += key_len + val_len;
        }
    }

    pub fn delete(&self, key: Key) {
        let mut map = self.map.write();
        let mut size = self.size.write();
        
        let key_len = key.len();
        
        if let Some(old) = map.insert(key, None) {
            if let Some(old_val) = old {
                *size = *size + 1 - old_val.len(); // None is effectively 1 byte or so in terms of overhead
            }
        } else {
            *size += key_len + 1;
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Option<Value>> {
        let map = self.map.read();
        map.get(key).cloned()
    }

    pub fn size(&self) -> usize {
        *self.size.read()
    }

    pub fn clear(&self) {
        let mut map = self.map.write();
        let mut size = self.size.write();
        map.clear();
        *size = 0;
    }

    pub fn entries(&self) -> Vec<(Key, Option<Value>)> {
        let map = self.map.read();
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}
