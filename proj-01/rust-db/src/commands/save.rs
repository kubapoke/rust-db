use std::fs;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;
use crate::errors::Error;

pub struct SaveCommand {
    path: String,
    commands: Vec<String>
}

impl SaveCommand {
    pub fn new(path: String, commands: Vec<String>) -> Self {
        Self { path, commands }
    }
}

impl Command for SaveCommand {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let commands = self.commands.join("\n");
        fs::write(self.path.clone(), commands)
            .map_err(|e| Error::IOError(format!("Failed to write to {}: {}", self.path, e)))?;
        Ok(ExecutionSuccessValue::SuccessFileOperation(format!("Saved {} commands", self.commands.len())))
    }
}
