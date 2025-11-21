use std::collections::HashMap;

pub trait DatabaseKey: Eq + Ord {}

impl DatabaseKey for String {}
impl DatabaseKey for i64 {}

pub enum FieldType {
    Bool,
    String,
    Int,
    Float,
}

pub enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

pub struct Record {
    values: HashMap<String, Value>,
}

pub struct Table<K: DatabaseKey> {
    key: String,
    fields: HashMap<String, FieldType>,
    records: HashMap<K, Record>,
}

pub struct Database<K: DatabaseKey> {
    tables: HashMap<String, Table<K>>,
}

pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}