use crate::commands::clauses::clause::{AnyClause, Clause};
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::key::DatabaseKey;
use crate::database::slice::SelectResult;
use crate::database::table::Table;
use crate::errors;

pub struct SelectCommand<'a, K: DatabaseKey> {
    table: &'a Table<K>,
    fields: Vec<String>,
    clauses: Vec<AnyClause>,
}

impl<'a, K: DatabaseKey> SelectCommand<'a, K> {
    pub fn new(table: &'a Table<K>, fields: Vec<String>, clauses: Vec<AnyClause>) -> Self {
        Self { table, fields, clauses }
    }
}

impl<K: DatabaseKey> Command for SelectCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let mut slice = self.table.to_slice();

        for clause in &self.clauses {
            slice = clause.apply(slice)?;
        }

        let mut result_rows = Vec::new();
        for record in slice.records {
            let projected = record.project_fields(&self.fields)?;
            result_rows.push(projected);
        }

        Ok(ExecutionSuccessValue::SelectResult(SelectResult { rows: result_rows }))
    }
}
