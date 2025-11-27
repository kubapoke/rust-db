use crate::commands::clauses::clause::Clause;
use crate::commands::clauses::evaluable::{AnyEvaluable, Evaluable};
use crate::database::slice::TableSlice;
use crate::errors::Error;

pub struct WhereClause<'a> {
    pub evaluation: AnyEvaluable<'a>,
}

impl WhereClause<'_> {
    pub fn new(evaluation: AnyEvaluable) -> WhereClause {
        WhereClause { evaluation }
    }
}

impl Clause for WhereClause<'_> {
    fn apply(&self, slice: TableSlice) -> Result<TableSlice, Error> {
        let mut filtered = Vec::new();

        for record in slice.records.into_iter() {
            if self.evaluation.evaluate(&record)? {
                filtered.push(record);
            }
        }

        Ok(TableSlice::new(filtered))
    }
}