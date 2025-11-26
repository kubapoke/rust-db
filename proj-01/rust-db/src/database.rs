use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors::Error;
use crate::parser::parse_command;

pub trait DatabaseKey: Eq + Ord {
    fn get_key_type() -> KeyType;
    fn get_field_type() -> FieldType;
}

impl DatabaseKey for String {
    fn get_key_type() -> KeyType {
        KeyType::String
    }

    fn get_field_type() -> FieldType {
        FieldType::String
    }
}
impl DatabaseKey for i64 {
    fn get_key_type() -> KeyType {
        KeyType::String
    }

    fn get_field_type() -> FieldType {
        FieldType::String
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyType {
    String,
    Int,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldType {
    Bool,
    String,
    Int,
    Float,
}

#[derive(Clone, Debug)]
pub enum IntermediateValue {
    Bool(bool),
    String(String),
    Numeric(f64),
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

#[derive(Clone, Debug)]
pub struct Record {
    values: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

    pub fn has_table(&self, name: &String) -> bool {
        self.tables.contains_key(name)
    }
    
    pub fn get_table(&self, name: &String) -> Result<&Table<K>, Error> {
        self.tables.get(name)
            .ok_or_else(|| Error::NotExistError(format!("Table '{}' name does not exist", name).to_string()))
    }
    
    pub fn execute_command(&mut self, command: &str) -> Result<ExecutionSuccessValue, Error> {
        let mut executable = parse_command(command, self)?;
        executable.execute()
    }
}

#[derive(Clone, Debug)]
pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}

impl AnyDatabase {
    pub fn new(key: KeyType) -> Self {
        match key {
            KeyType::String => Self::StringDatabase(Database::new()),
            KeyType::Int => Self::IntDatabase(Database::new()),
        }
    }

    pub fn key_type(&self) -> FieldType {
        match self {
            AnyDatabase::StringDatabase(_) => FieldType::String,
            AnyDatabase::IntDatabase(_) => FieldType::Int,
        }
    }

    pub fn execute_command(&mut self, command: &str) -> Result<ExecutionSuccessValue, Error> {
        match self {
            AnyDatabase::StringDatabase(db) => db.execute_command(command),
            AnyDatabase::IntDatabase(db) => db.execute_command(command),
        }
    }
}
