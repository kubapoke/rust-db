use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors::Error;
use crate::parser::parse_command;

pub trait DatabaseKey: Eq + Ord + Sized + Hash {
    fn get_key_type() -> KeyType;
    fn get_field_type() -> FieldType;
    fn from_value(v: &Value) -> Result<Self, Error>;
}

impl DatabaseKey for String {
    fn get_key_type() -> KeyType {
        KeyType::String
    }

    fn get_field_type() -> FieldType {
        FieldType::String
    }

    fn from_value(v: &Value) -> Result<Self, Error> {
        match v {
            Value::String(s) => Ok(s.clone()),
            _ => Err(Error::TypeError("Expected string key".into()))
        }
    }
}
impl DatabaseKey for i64 {
    fn get_key_type() -> KeyType {
        KeyType::String
    }

    fn get_field_type() -> FieldType {
        FieldType::String
    }

    fn from_value(v: &Value) -> Result<Self, Error> {
        match v {
            Value::Int(i) => Ok(*i),
            _ => Err(Error::TypeError("Expected integer key".into()))
        }
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

impl IntermediateValue {
    pub fn to_value(&self, field_type: &FieldType) -> Result<Value, Error> {
        match (field_type, self) {
            (FieldType::Bool, IntermediateValue::Bool(b)) =>
                Ok(Value::Bool(*b)),
            (FieldType::String, IntermediateValue::String(s)) =>
                Ok(Value::String(s.clone())),
            (FieldType::Int, IntermediateValue::Numeric(n)) => {
                if *n == (*n as i64 as f64) {
                    Ok(Value::Int(*n as i64))
                } else {
                    Err(Error::TypeError(format!("Expected integer but got non-integer number '{}'", n)))
                }
            }
            (FieldType::Float, IntermediateValue::Numeric(n)) =>
                Ok(Value::Float(*n)),
            (expected, got) =>
                Err(Error::TypeError(format!("Field expects {:?} but record provides {:?}", expected, got))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

impl Value {
    fn get_field_type(&self) -> FieldType {
        match self {
            Value::Bool(_) => FieldType::Bool,
            Value::String(_) => FieldType::String,
            Value::Int(_) => FieldType::Int,
            Value::Float(_) => FieldType::Float,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntermediateRecord {
    pub(crate) values: HashMap<String, IntermediateValue>,
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
    
    pub fn key_type(&self) -> Result<FieldType, Error> {
        self.fields.get(&self.key)
            .ok_or_else(|| Error::NotSpecifiedError("Field type of key was not specified".into()))
            .cloned()
    }

    fn validate_new_key(&self, key_value: &Value, typed_key: &K) -> Result<(), Error> {
        if key_value.get_field_type() != K::get_field_type() {
            return Err(Error::TypeError("Mismatched field type".to_string()));
        }

        if self.records.contains_key(typed_key) {
            return Err(Error::AlreadyExistsError("Key already exists".into()));
        }

        Ok(())
    }

    pub fn convert_record(&self, intermediate: IntermediateRecord) -> Result<Record, Error> {
        let mut values = HashMap::new();

        for (field_name, field_type) in &self.fields {
            let intermediate_value = intermediate.values.get(field_name)
                .ok_or_else(|| Error::MissingFieldError(format!("Record missing required field '{}'", field_name)))?;

            let value = intermediate_value.to_value(field_type)?;
            values.insert(field_name.clone(), value);
        }

        for field_name in intermediate.values.keys() {
            if !self.fields.contains_key(field_name) {
                return Err(Error::NotExistError(format!("Unexpected field '{}'", field_name)));
            }
        }

        Ok(Record { values })
    }

    pub fn add_record(&mut self, record: IntermediateRecord) -> Result<(), Error> {
        let record = self.convert_record(record)?;

        let key_value = record.values.get(&self.key)
            .ok_or_else(|| Error::MissingFieldError("The record is missing the key value".to_string()))?;

        let typed_key = K::from_value(key_value)?;

        self.validate_new_key(&key_value, &typed_key)?;

        self.records.insert(typed_key, record);

        Ok(())
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
            return Err(Error::AlreadyExistsError(format!("Table '{}' already exists", name)));
        }
        
        if table.key_type()? != K::get_field_type() {
            return Err(Error::TypeError("Mismatched key type".to_string()));
        }

        self.tables.insert(name, table);
        Ok(())
    }

    pub fn has_table(&self, name: &String) -> bool {
        self.tables.contains_key(name)
    }

    pub fn get_table(&mut self, name: &String) -> Result<&mut Table<K>, Error> {
        self.tables.get_mut(name)
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
