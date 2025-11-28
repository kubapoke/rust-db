use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::database::key::DatabaseKey;
use crate::database::table::Table;
use crate::database::value::KeyValue;
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
        
        Ok(ExecutionSuccessValue::Success(format!("Successfully deleted record {} from table", typed_key)))
    }
}

#[cfg(test)]
mod tests{
    use crate::commands::create::CreateCommand;
    use crate::commands::insert::InsertCommand;
    use crate::database::databases::Database;
    use crate::database::types::FieldType;
    use crate::database::value::IntermediateValue;
    use super::*;

    #[test]
    fn test_delete_command() {
        let mut db = Database::<String>::new();

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
                ("year".to_string(), IntermediateValue::Numeric(2000f64)),
            ]
        );
        cmd.execute().unwrap();

        let mut cmd = DeleteCommand::new(
            db.get_table(&"library".to_string()).unwrap(),
            KeyValue::String("1".to_string())
        );

        let result = cmd.execute().unwrap();

        assert!(matches!(result, ExecutionSuccessValue::Success(_)));
        assert_eq!(db.get_table(&"library".to_string()).unwrap().len(), 0)
    }
}
