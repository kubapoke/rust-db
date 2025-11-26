use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{DatabaseKey, IntermediateValue, Table};
use crate::errors;

pub struct InsertCommand<'a, K: DatabaseKey> {
    table: &'a Table<K>,
    fields: Vec<(String, IntermediateValue)>,
}

impl<'a, K: DatabaseKey> InsertCommand<'a, K> {
    pub fn new(table: &'a Table<K>, fields: Vec<(String, IntermediateValue)>) -> Self {
        Self { table, fields }
    }
}

impl<K: DatabaseKey> Command for InsertCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
