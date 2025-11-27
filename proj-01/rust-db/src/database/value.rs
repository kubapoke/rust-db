use std::cmp::Ordering;
use crate::database::types::FieldType;
use crate::errors::Error;
use crate::errors::Error::InvalidComparisonError;

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

pub fn compare_values(a: &Option<&Value>, b: &Option<&Value>) -> Ordering {
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
