use std::collections::HashMap;

pub trait DatabaseKey: Eq + Ord {}

impl DatabaseKey for String {}
impl DatabaseKey for i64 {}

pub enum KeyType {
    String,
    Int,
}

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

impl<K: DatabaseKey> Database<K> {
    pub fn new() -> Self {
        Self { tables: HashMap::new(), }
    }
}

pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}

pub fn get_database(key: KeyType) -> AnyDatabase {
    match key {
        KeyType::String => {
            let db = Database::<String>::new();
            AnyDatabase::StringDatabase(db)
        }
        KeyType::Int => {
            let db = Database::<i64>::new();
            AnyDatabase::IntDatabase(db)
        }
    }
}
