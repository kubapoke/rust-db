use crate::commands::clauses::limit::LimitClause;
use crate::commands::clauses::order::OrderByClause;
use crate::commands::clauses::r#where::WhereClause;
use crate::database::slice::TableSlice;
use crate::errors::Error;

pub trait Clause {
    fn apply(&self, slice: TableSlice) -> Result<TableSlice, Error>;
}

pub enum AnyClause<'a> {
    Where(WhereClause<'a>),
    Order(OrderByClause),
    Limit(LimitClause),
}

impl Clause for AnyClause<'_> {
    fn apply(&self, slice: TableSlice) -> Result<TableSlice, Error> {
        match self {
            Self::Where(clause) => clause.apply(slice),
            Self::Order(clause) => clause.apply(slice),
            Self::Limit(clause) => clause.apply(slice),
        }
    }
}