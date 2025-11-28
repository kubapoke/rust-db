use std::fs;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::database::{AnyDatabase, Database};
use crate::database::key::DatabaseKey;
use crate::errors;
use crate::errors::Error;

pub struct ReadCommand<'a, K: DatabaseKey> {
    pub database: &'a mut Database<K>,
    path: String,
}

impl<'a, K: DatabaseKey> ReadCommand<'a, K> {
    pub fn new(database: &'a mut Database<K>, path: String) -> Self {
        Self { database, path }
    }
}

impl<'a, K: DatabaseKey> Command for ReadCommand<'a, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let content = fs::read_to_string(&self.path)
            .map_err(|e| Error::IOError(format!("Failed to read {}: {}", self.path, e)))?;

        for line in content.lines() {
            if line.trim().is_empty() { continue; }
            self.database.execute_command(line.trim())?;
        }

        Ok(ExecutionSuccessValue::SuccessFileOperation(format!("Executed commands from {}", self.path)))
    }
}
