use crate::commands::clauses::clause::Clause;
use crate::database::slice::TableSlice;
use crate::errors::Error;

pub struct  LimitClause {
    pub amount: usize
}

impl LimitClause {
    pub fn new(amount: usize) -> Self {
        Self { amount }
    }
}

impl Clause for LimitClause {
    fn apply(&self, mut slice: TableSlice) -> Result<TableSlice, Error> {
        slice.records.truncate(self.amount);
        Ok(slice)
    }
}