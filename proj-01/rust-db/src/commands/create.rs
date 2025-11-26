use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{AnyDatabase, FieldType, Table};
use crate::errors;
use crate::errors::Error;

pub struct CreateCommand<'a> {
    database: &'a  mut AnyDatabase,
    name: String,
    key: String,
    fields: Vec<(String, FieldType)>,
}

impl<'a> CreateCommand<'a> {
    pub fn new(database: &'a mut AnyDatabase, name: String, key: String, fields: Vec<(String, FieldType)>) -> Self {
        Self { database, name, key, fields }
    }

    fn create_fields(&self) -> Result<HashMap<String, FieldType>, Error> {
        let mut fields = HashMap::new();

        for (name, field_type) in &self.fields {
            if fields.contains_key(name) {
                return Err(Error::AlreadyExistsError(format!("Field '{}' is declared more than once", name)))
            }

            fields.insert(name.clone(), field_type.clone());
        }

        Ok(fields)
    }

    fn validate_key(&self, fields: &HashMap<String, FieldType>) -> Result<(), Error> {
        let key_field_type = fields.get(&self.key)
            .ok_or_else(|| Error::NotSpecifiedError("Field type of key was not specified".into()))?;

        if *key_field_type != self.database.key_type() {
            return Err(Error::TypeError("Invalid key field type".into()));
        }

        Ok(())
    }
}

impl Command for CreateCommand<'_> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let key = self.key.clone();
        let fields = self.create_fields()?;
        
        self.validate_key(&fields)?;

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
