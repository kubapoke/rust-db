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

#[cfg(test)]
mod tests {
    use crate::commands::clauses::evaluable::{AnyEvaluable, CompOp, Comparison, ComparisonOr};
    use crate::commands::clauses::limit::LimitClause;
    use crate::commands::clauses::order::OrderByClause;
    use crate::commands::clauses::r#where::WhereClause;
    use crate::commands::create::CreateCommand;
    use crate::commands::insert::InsertCommand;
    use crate::database::databases::Database;
    use crate::database::types::FieldType;
    use crate::database::value::{IntermediateValue, Value};
    use super::*;

    fn prepare_db(mut db: &mut Database::<String>) {
        let mut cmd = CreateCommand::new(
            &mut db,
            "library".to_string(),
            "id".to_string(),
            vec![
                ("id".to_string(), FieldType::String),
                ("year".to_string(), FieldType::Int)
            ]
        );
        cmd.execute().unwrap();

        let mut cmd = InsertCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            vec![
                ("id".to_string(), IntermediateValue::String("1".to_string())),
                ("year".to_string(), IntermediateValue::Numeric(2002f64)),
            ]
        );
        cmd.execute().unwrap();

        let mut cmd = InsertCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            vec![
                ("id".to_string(), IntermediateValue::String("2".to_string())),
                ("year".to_string(), IntermediateValue::Numeric(2001f64)),
            ]
        );
        cmd.execute().unwrap();

        let mut cmd = InsertCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            vec![
                ("id".to_string(), IntermediateValue::String("3".to_string())),
                ("year".to_string(), IntermediateValue::Numeric(2000f64)),
            ]
        );
        cmd.execute().unwrap();
    }

    #[test]
    fn test_select_command() {
        let mut db = Database::<String>::new();
        prepare_db(&mut db);

        let mut cmd = SelectCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            vec!["id".to_string(), "year".to_string()],
            vec![]
        );

        let result = cmd.execute().unwrap();

        assert!(matches!(result, ExecutionSuccessValue::SelectResult(_)));

        if let ExecutionSuccessValue::SelectResult(r) = result {
            assert_eq!(r.rows.len(), 3);
        }
    }
    
    #[test]
    fn test_select_where() {
        let mut db = Database::<String>::new();
        prepare_db(&mut db);

        let mut cmd = SelectCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            vec!["id".to_string(), "year".to_string()],
            vec![AnyClause::Where(WhereClause::new(
                AnyEvaluable::Or(
                    ComparisonOr::new(
                        *Box::new(AnyEvaluable::Comp(
                            Comparison::new("id".to_string(), CompOp::Eq, IntermediateValue::String("1".to_string()))
                        )),
                        *Box::new(AnyEvaluable::Comp(
                            Comparison::new("year".to_string(), CompOp::Lt, IntermediateValue::Numeric(2001f64)),
                        ))
                    )
                )
            ))]
        );

        let result = cmd.execute().unwrap();

        assert!(matches!(result, ExecutionSuccessValue::SelectResult(_)));

        if let ExecutionSuccessValue::SelectResult(r) = result {
            assert_eq!(r.rows.len(), 2);
        }
    }

    #[test]
    fn test_select_order() {
        let mut db = Database::<String>::new();
        prepare_db(&mut db);

        let mut cmd = SelectCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            vec!["id".to_string(), "year".to_string()],
            vec![AnyClause::Order(OrderByClause::new(vec!["year".to_string()]))]
        );

        let result = cmd.execute().unwrap();

        assert!(matches!(result, ExecutionSuccessValue::SelectResult(_)));

        if let ExecutionSuccessValue::SelectResult(r) = result {
            assert_eq!(r.rows.len(), 3);
            if let Value::String(id) = &r.rows[0].values[0].1 {
                assert_eq!(id, "3")
            }
            if let Value::String(id) = &r.rows[1].values[0].1 {
                assert_eq!(id, "2")
            }
            if let Value::String(id) = &r.rows[2].values[0].1 {
                assert_eq!(id, "1")
            }
        }
    }

    #[test]
    fn test_select_limit() {
        let mut db = Database::<String>::new();
        prepare_db(&mut db);

        let mut cmd = SelectCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            vec!["id".to_string(), "year".to_string()],
            vec![AnyClause::Limit(LimitClause::new(1))]
        );

        let result = cmd.execute().unwrap();

        assert!(matches!(result, ExecutionSuccessValue::SelectResult(_)));

        if let ExecutionSuccessValue::SelectResult(r) = result {
            assert_eq!(r.rows.len(), 1);
        }
    }
}
