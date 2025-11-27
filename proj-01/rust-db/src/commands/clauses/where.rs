use crate::commands::clauses::clause::Clause;
use crate::database::slice::TableSlice;
use crate::errors::Error;

pub struct WhereClause {
    
}

impl Clause for WhereClause {
    fn apply(&self, slice: TableSlice) -> Result<TableSlice, Error> {
        todo!()
    }
}