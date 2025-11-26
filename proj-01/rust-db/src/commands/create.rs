use std::collections::HashMap;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{AnyDatabase, FieldType};
use crate::errors;

pub struct CreateCommand {
    database: &'static AnyDatabase,
    name: String,
    key: String,
    fields: HashMap<String, FieldType>,
}

impl CreateCommand {
    pub fn new(database: &'static AnyDatabase, name: String, key: String, fields: HashMap<String, FieldType>) -> Self {
        Self { database, name, key, fields }
    }
}

impl Command for CreateCommand {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
