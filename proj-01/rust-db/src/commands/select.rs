use crate::commands::clauses::clause::{AnyClause, Clause};
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::key::DatabaseKey;
use crate::database::table::Table;
use crate::errors;

pub struct SelectCommand<'a, K: DatabaseKey> {
    table: &'a Table<K>,
    clauses: Vec<AnyClause<'a>>,
}

impl<K: DatabaseKey> Command for SelectCommand<'_, K> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let mut slice = self.table.to_slice();
        
        for clause in &self.clauses {
            slice = clause.apply(slice)?;
        }
        
        todo!()
    }
}
