use std::collections::HashMap;
use crate::database::value::Value;

pub struct Record {
    values: HashMap<String, Value>,
}