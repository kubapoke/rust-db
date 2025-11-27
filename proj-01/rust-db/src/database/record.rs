use std::collections::HashMap;
use crate::database::value::{IntermediateValue, Value};

#[derive(Clone, Debug)]
pub struct IntermediateRecord {
    pub(crate) values: HashMap<String, IntermediateValue>,
}

#[derive(Clone, Debug)]
pub struct Record {
    pub(crate) values: HashMap<String, Value>,
}