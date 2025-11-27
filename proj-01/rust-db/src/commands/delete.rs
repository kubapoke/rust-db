use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{DatabaseKey, Table};
use crate::errors;

pub struct DeleteCommand<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    key: String
}

impl<K: DatabaseKey> Command for DeleteCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
