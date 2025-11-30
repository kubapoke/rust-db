use std::collections::HashMap;
use crate::database::slice::SelectRow;
use crate::database::value::{IntermediateValue, Value};
use crate::errors::Error;

#[derive(Clone, Debug)]
pub struct IntermediateRecord {
    pub values: HashMap<String, IntermediateValue>,
}

#[derive(Clone, Debug)]
pub struct Record {
    pub values: HashMap<String, Value>,
}

impl Record {
    pub fn project_fields(&self, fields: &[String]) -> Result<SelectRow, Error> {
        let mut out = Vec::new();

        for field in fields {
            let val = self.values.get(field)
                .ok_or_else(|| Error::MissingFieldError(format!("Field '{}' does not exist", field)))?;

            out.push((field.clone(), val.clone()));
        }

        Ok(SelectRow { values: out })
    }
}
