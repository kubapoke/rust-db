use std::fs;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::database::{Database};
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

    fn execute_command(&mut self, command_str: &String) -> Result<(), Error> {
        println!("{}", command_str);
        let result = self.database.execute_command(command_str)?;

        match result {
            ExecutionSuccessValue::Success(msg) => println!("{}\n", msg),
            ExecutionSuccessValue::SuccessFileOperation(msg) => println!("{}\n", msg),
            ExecutionSuccessValue::SelectResult(res) => println!("{}\n", res.to_string()),
        }

        Ok(())
    }
}

impl<'a, K: DatabaseKey> Command for ReadCommand<'a, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let content = fs::read_to_string(&self.path)
            .map_err(|e| Error::IOError(format!("Failed to read {}: {}", self.path, e)))?;

        let mut lines = content.lines();
        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }

            let command_str = if trimmed.starts_with("CREATE") {
                match lines.next() {
                    Some(next_line) => format!("{}\n{}", trimmed, next_line.trim()),
                    None => { return Err(Error::IOError("CREATE command requires FIELDS after a newline".to_string())); },
                }
            } else {
                trimmed.to_string()
            };

            self.execute_command(&command_str)?;
        }
        Ok(ExecutionSuccessValue::SuccessFileOperation(format!("Executed commands from {}", self.path)))
    }
}
