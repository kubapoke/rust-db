use crate::commands::clauses::clause::Clause;
use crate::database::slice::TableSlice;
use crate::database::value::compare_values;
use crate::errors::Error;

pub struct OrderByClause {
    pub fields: Vec<String>,
}

impl OrderByClause {
    pub fn new(fields: Vec<String>) -> OrderByClause {
        OrderByClause { fields }
    }
}

impl Clause for OrderByClause {
    fn apply(&self, mut slice: TableSlice) -> Result<TableSlice, Error> {
        slice.records.sort_by(|a, b| {
            for field in &self.fields {
                let a = a.values.get(field);
                let b = b.values.get(field);

                let cmp = compare_values(&a, &b);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            std::cmp::Ordering::Equal
        });

        Ok(slice)
    }
}