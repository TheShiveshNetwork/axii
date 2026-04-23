use serde::{Deserialize, Serialize};

pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operation {
    Set(Key, Value),
    Delete(Key),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub key: Key,
    pub value: Option<Value>, // None represents a tombstone
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Get(Key),
    Set(Key, Value),
    Delete(Key),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Ok(Option<Value>),
    Error(String),
}
