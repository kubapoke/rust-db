use std::fs;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;
use crate::errors::Error;

pub struct SaveCommand<'a> {
    path: String,
    commands: &'a Vec<String>
}

impl<'a> SaveCommand<'a> {
    pub fn new(path: String, commands: &'a Vec<String>) -> Self {
        Self { path, commands }
    }
}

impl Command for SaveCommand<'_> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let commands = self.commands.join("\n");
        fs::write(self.path.clone(), commands)
            .map_err(|e| Error::IOError(format!("Failed to write to {}: {}", self.path, e)))?;
        Ok(ExecutionSuccessValue::SuccessFileOperation(format!("Saved {} commands", self.commands.len())))
    }
}
