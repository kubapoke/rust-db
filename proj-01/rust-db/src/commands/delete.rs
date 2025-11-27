use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::{DatabaseKey, KeyValue, Table};
use crate::errors;

pub struct DeleteCommand<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    key: KeyValue
}

impl<'a, K: DatabaseKey> DeleteCommand<'a, K> {
    pub fn new(table: &'a mut Table<K>, key: KeyValue) -> Self {
        Self { table, key }
    }
}

impl<K: DatabaseKey> Command for DeleteCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let typed_key = K::from_key_value(&self.key)?;
        self.table.delete_record(&typed_key)?;
        
        Ok(ExecutionSuccessValue::Success)
    }
}
