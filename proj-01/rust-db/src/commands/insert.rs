use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::key::DatabaseKey;
use crate::database::record::IntermediateRecord;
use crate::database::table::Table;
use crate::database::value::IntermediateValue;
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
        
        Ok(ExecutionSuccessValue::Success("Successfully inserted record".to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::commands::create::CreateCommand;
    use crate::database::databases::Database;
    use crate::database::types::FieldType;
    use super::*;

    #[test]
    fn test_insert_command() {
        let mut db = Database::<String>::new();

        let mut cmd = CreateCommand::new(
            &mut db,
            "library".to_string(),
            "id".to_string(),
            vec![
                ("id".to_string(), FieldType::String),
                ("year".to_string(), FieldType::Int)
            ]
        );
        cmd.execute().unwrap();

        let mut cmd = InsertCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            vec![
                ("id".to_string(), IntermediateValue::String("1".to_string())),
                ("year".to_string(), IntermediateValue::Numeric(2000f64)),
            ]
        );

        let result = cmd.execute().unwrap();

        assert!(matches!(result, ExecutionSuccessValue::Success(_)));
        assert_eq!(db.get_table(&"library".to_string()).unwrap().len(), 1)
    }
}
