use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{DatabaseKey, IntermediateRecord, IntermediateValue, Table};
use crate::errors;
use crate::errors::Error;

pub struct InsertCommand<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    assignments: Vec<(String, IntermediateValue)>,
}

impl<'a, K: DatabaseKey> InsertCommand<'a, K> {
    pub fn new(table: &'a mut Table<K>, fields: Vec<(String, IntermediateValue)>) -> Self {
        Self { table, assignments: fields }
    }
    
    pub fn create_assignments(&self) -> Result<HashMap<String, IntermediateValue>, Error> {
        let mut assignments_map = HashMap::new();

        for (name, value) in &self.assignments {
            if assignments_map.contains_key(name) {
                return Err(Error::AlreadyExistsError(format!("Field '{}' is assigned more than once", name)));
            }
            assignments_map.insert(name.clone(), value.clone());
        }

        Ok(assignments_map)
    }
}

impl<K: DatabaseKey> Command for InsertCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let assignments = self.create_assignments()?;
        
        let record = IntermediateRecord { values: assignments };
        
        self.table.add_record(record)?;
        
        Ok(ExecutionSuccessValue::Success)
    }
}
