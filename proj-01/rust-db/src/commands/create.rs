use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::databases::Database;
use crate::database::key::DatabaseKey;
use crate::database::table::Table;
use crate::database::types::FieldType;
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
}

impl<K: DatabaseKey> Command for CreateCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let key = self.key.clone();
        let fields = self.create_fields()?;

        let table = Table::new(key, fields, HashMap::new());

        self.database.add_table(self.name.clone(), table)?;

        Ok(ExecutionSuccessValue::Success(format!("Table '{}' created successfully", self.name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::command::{Command, ExecutionSuccessValue};

    #[test]
    fn test_create() {
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

        let result = cmd.execute().unwrap();
        assert!(matches!(result, ExecutionSuccessValue::Success(_)));
        assert!(db.get_table(&"library".to_string()).is_ok());
    }

    #[test]
    fn test_create_command() {
        let mut db = Database::<String>::new();

        let cmd = "CREATE library KEY id
        FIELDS id: String, year: Int";

        let result = db.execute_command(cmd);

        assert!(matches!(result, Ok(_)));
        assert!(db.get_table(&"library".to_string()).is_ok());
    }
}
