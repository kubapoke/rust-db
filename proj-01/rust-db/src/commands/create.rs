use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{AnyDatabase, DatabaseKey, FieldType, Table};
use crate::errors;

pub struct CreateCommand {
    database: &'static mut AnyDatabase,
    name: String,
    key: String,
    fields: HashMap<String, FieldType>,
}

impl CreateCommand {
    pub fn new(database: &'static mut AnyDatabase, name: String, key: String, fields: HashMap<String, FieldType>) -> Self {
        Self { database, name, key, fields }
    }
}

impl Command for CreateCommand {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let key = self.key.clone();
        let fields = self.fields.clone();

        match self.database {
            AnyDatabase::StringDatabase(db) => {
                let table = Table::new(key, fields, HashMap::new());
                db.add_table(self.name.clone(), table)?;
            }
            AnyDatabase::IntDatabase(db) => {
                let table = Table::new(key, fields, HashMap::new());
                db.add_table(self.name.clone(), table)?
            }
        }

        Ok(ExecutionSuccessValue::Success)
    }
}
