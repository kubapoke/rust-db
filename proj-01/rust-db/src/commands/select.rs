use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{DatabaseKey, Table};
use crate::errors;

pub struct SelectCommand<'a, K: DatabaseKey> {
    table: &'a Table<K>
}

impl<K: DatabaseKey> Command for SelectCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
