use std::collections::HashMap;
use crate::errors::Error;

pub trait DatabaseKey: Eq + Ord {}

impl DatabaseKey for String {}
impl DatabaseKey for i64 {}

pub enum KeyType {
    String,
    Int,
}

#[derive(Clone, Debug)]
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

impl<K: DatabaseKey> Table<K> {
    pub fn new(key: String, fields: HashMap<String, FieldType>, records: HashMap<K, Record>) -> Self {
        Table { key, fields, records }
    }
}

pub struct Database<K: DatabaseKey> {
    tables: HashMap<String, Table<K>>,
}

impl<K: DatabaseKey> Database<K> {
    pub fn new() -> Self {
        Self { tables: HashMap::new(), }
    }
    
    pub fn add_table(&mut self, name: String, table: Table<K>) -> Result<(), Error> {
        if self.tables.contains_key(&name) {
            return Err(Error::AlreadyExistsError(name))
        }
        
        self.tables.insert(name, table);
        
        Ok(())
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
