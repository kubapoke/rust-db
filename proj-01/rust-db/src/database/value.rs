use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::database::types::FieldType;
use crate::errors::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyValue {
    String(String),
    Int(i64),
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
    pub fn get_field_type(&self) -> FieldType {
        match self {
            Value::Bool(_) => FieldType::Bool,
            Value::String(_) => FieldType::String,
            Value::Int(_) => FieldType::Int,
            Value::Float(_) => FieldType::Float,
        }
    }

    fn value_order(&self) -> u8 {
        match self {
            Value::Bool(_) => 0,
            Value::String(_) => 1,
            Value::Int(_) => 2,
            Value::Float(_) => 3,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
        }
    }
}

pub fn sort_compare_values(a: &Option<&Value>, b: &Option<&Value>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(Value::Bool(x)), Some(Value::Bool(y))) => x.cmp(y),
        (Some(Value::String(x)), Some(Value::String(y))) => x.cmp(y),
        (Some(Value::Int(x)), Some(Value::Int(y))) => x.cmp(y),
        (Some(Value::Float(x)), Some(Value::Float(y))) => x.partial_cmp(y).unwrap_or_else(|| Ordering::Equal),
        (Some(va), Some(vb)) => va.value_order().cmp(&vb.value_order()),
    }
}

pub fn compare_value_intermediate_value(value: &Value, constant: &IntermediateValue) -> Result<Ordering, Error> {
    match (value, constant) {
        (Value::Int(a), IntermediateValue::Numeric(b)) => Ok((*a as f64).partial_cmp(b).ok_or_else(|| Error::TypeError("".to_string()))?),
        (Value::Float(a), IntermediateValue::Numeric(b)) => Ok(a.partial_cmp(b).ok_or_else(|| Error::TypeError("numeric value".to_string()))?),
        (Value::Bool(a), IntermediateValue::Bool(b)) => Ok(a.cmp(b)),
        (Value::String(a), IntermediateValue::String(b)) => Ok(a.cmp(b)),
        (a, b) => Err(Error::TypeError(format!("Cannot compare field value {:?} with constant {:?}", a, b))),
    }
}
