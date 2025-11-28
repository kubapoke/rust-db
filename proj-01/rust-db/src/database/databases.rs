use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::key::DatabaseKey;
use crate::database::table::Table;
use crate::database::types::{FieldType, KeyType};
use crate::errors::Error;
use crate::parser::parse_command;

#[derive(Clone, Debug)]
pub struct Database<K: DatabaseKey> {
    tables: HashMap<String, Table<K>>,
    session_commands: Vec<String>
}

impl<K: DatabaseKey> Database<K> {
    pub fn new() -> Self {
        Self { tables: HashMap::new(), session_commands: Vec::new() }
    }

    pub fn add_table(&mut self, name: String, table: Table<K>) -> Result<(), Error> {
        if self.tables.contains_key(&name) {
            return Err(Error::AlreadyExistsError(format!("Table '{}' already exists", name)));
        }

        if table.key_type()? != K::get_field_type() {
            return Err(Error::TypeError("Mismatched key type".to_string()));
        }

        self.tables.insert(name, table);
        Ok(())
    }

    pub fn has_table(&self, name: &String) -> bool {
        self.tables.contains_key(name)
    }

    pub fn get_table(&mut self, name: &String) -> Result<&mut Table<K>, Error> {
        self.tables.get_mut(name)
            .ok_or_else(|| Error::NotExistError(format!("Table '{}' name does not exist", name).to_string()))
    }

    pub fn execute_command(&mut self, command: &str) -> Result<ExecutionSuccessValue, Error> {
        let mut executable = parse_command(command, self)?;
        executable.execute()
    }
    
    pub fn get_session_commands(&self) -> &Vec<String> {
        &self.session_commands
    }
}

impl<K: DatabaseKey> Default for Database<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}

impl AnyDatabase {
    pub fn new(key: KeyType) -> Self {
        match key {
            KeyType::String => Self::StringDatabase(Database::new()),
            KeyType::Int => Self::IntDatabase(Database::new()),
        }
    }

    pub fn key_type(&self) -> FieldType {
        match self {
            AnyDatabase::StringDatabase(_) => FieldType::String,
            AnyDatabase::IntDatabase(_) => FieldType::Int,
        }
    }

    pub fn execute_command(&mut self, command: &str) -> Result<ExecutionSuccessValue, Error> {
        let result = match self {
            AnyDatabase::StringDatabase(db) => db.execute_command(command)?,
            AnyDatabase::IntDatabase(db) => db.execute_command(command)?,
        };

        if !matches!(result, ExecutionSuccessValue::SuccessFileOperation(_)) {
            match self {
                AnyDatabase::StringDatabase(db) => db.session_commands.push(command.to_string()),
                AnyDatabase::IntDatabase(db) => db.session_commands.push(command.to_string()),
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::table::Table;

    #[test]
    fn test_add_table() {
        let mut db: Database<i64> = Database::new();

        let table = Table::new(
            "id".to_string(),
            HashMap::from([("id".to_string(), FieldType::Int)]),
            HashMap::new(),
        );

        assert!(db.add_table("users".to_string(), table.clone()).is_ok());
        assert!(db.has_table(&"users".to_string()));

        let duplicate = db.add_table("users".to_string(), table);
        assert!(matches!(duplicate, Err(Error::AlreadyExistsError(_))));
    }

    #[test]
    fn test_any_database_execute_session_commands() {
        let mut db = AnyDatabase::new(KeyType::Int);

        let result = db.execute_command("CREATE users KEY id
            FIELDS id: Int");

        assert!(result.is_ok());

        if let AnyDatabase::IntDatabase(inner) = db {
            assert_eq!(inner.get_session_commands().len(), 1);
            assert!(inner.has_table(&"users".to_string()))
        }
    }
}
