use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{Database, DatabaseKey, FieldType, Table};
use crate::errors::{self, Error};

pub struct CreateCommand<'a, K: DatabaseKey> {
    database: &'a mut Database<K>,
    name: String,
    key: String,
    fields: Vec<(String, FieldType)>,
}

impl<'a, K: DatabaseKey> CreateCommand<'a, K> {
    pub fn new(database: &'a mut Database<K>, name: String, key: String, fields: Vec<(String, FieldType)>) -> Self {
        Self { database, name, key, fields }
    }

    fn create_fields(&self) -> Result<HashMap<String, FieldType>, Error> {
        let mut fields_map = HashMap::new();

        for (name, field_type) in &self.fields {
            if fields_map.contains_key(name) {
                return Err(Error::AlreadyExistsError(format!("Field '{}' is declared more than once", name)));
            }
            fields_map.insert(name.clone(), field_type.clone());
        }

        Ok(fields_map)
    }

    fn validate_key_type(&self, fields: &HashMap<String, FieldType>) -> Result<(), Error> {
        let key_field_type = fields.get(&self.key)
            .ok_or_else(|| Error::NotSpecifiedError("Field type of key was not specified".into()))?;

        if K::get_field_type() != *key_field_type {
            return Err(Error::TypeError("Invalid key field type".into()));
        }

        Ok(())
    }
}

impl<K: DatabaseKey> Command for CreateCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let key = self.key.clone();
        let fields = self.create_fields()?;

        self.validate_key_type(&fields)?;

        let table = Table::new(key, fields, HashMap::new());

        self.database.add_table(self.name.clone(), table)?;

        Ok(ExecutionSuccessValue::Success)
    }
}
