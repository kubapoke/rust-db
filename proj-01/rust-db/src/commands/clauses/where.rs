use crate::commands::clauses::clause::Clause;
use crate::commands::clauses::evaluable::AnyEvaluable;
use crate::database::slice::TableSlice;
use crate::errors::Error;

pub struct WhereClause<'a> {
    pub evaluation: AnyEvaluable<'a>,
}

impl Clause for WhereClause<'_> {
    fn apply(&self, slice: TableSlice) -> Result<TableSlice, Error> {
        todo!()
    }
}