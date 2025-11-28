use std::fmt::Display;
use std::hash::Hash;
use crate::database::types::{FieldType, KeyType};
use crate::database::value::{KeyValue, Value};
use crate::errors::Error;

pub trait DatabaseKey: Eq + Ord + Sized + Hash + Display {
    fn get_key_type() -> KeyType;
    fn get_field_type() -> FieldType;
    fn from_value(v: &Value) -> Result<Self, Error>;
    fn from_key_value(v: &KeyValue) -> Result<Self, Error>;
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
            _ => Err(Error::TypeError("Expected string key".to_string()))
        }
    }

    fn from_key_value(v: &KeyValue) -> Result<Self, Error> {
        match v {
            KeyValue::String(s) => Ok(s.clone()),
            _ => Err(Error::TypeError("Expected string key".to_string()))
        }
    }
}
impl DatabaseKey for i64 {
    fn get_key_type() -> KeyType {
        KeyType::Int
    }

    fn get_field_type() -> FieldType {
        FieldType::Int
    }

    fn from_value(v: &Value) -> Result<Self, Error> {
        match v {
            Value::Int(i) => Ok(*i),
            _ => Err(Error::TypeError("Expected integer key".to_string()))
        }
    }

    fn from_key_value(v: &KeyValue) -> Result<Self, Error> {
        match v {
            KeyValue::Int(i) => Ok(*i),
            _ => Err(Error::TypeError("Expected integer key".to_string()))
        }
    }
}
