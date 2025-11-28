use std::collections::HashMap;
use crate::database::key::DatabaseKey;
use crate::database::record::{IntermediateRecord, Record};
use crate::database::slice::TableSlice;
use crate::database::types::FieldType;
use crate::database::value::Value;
use crate::errors::Error;

#[derive(Clone, Debug)]
pub struct Table<K: DatabaseKey> {
    key: String,
    fields: HashMap<String, FieldType>,
    records: HashMap<K, Record>,
}

impl<K: DatabaseKey> Table<K> {
    pub fn new(key: String, fields: HashMap<String, FieldType>, records: HashMap<K, Record>) -> Self {
        Table { key, fields, records }
    }

    pub fn key_type(&self) -> Result<FieldType, Error> {
        self.fields.get(&self.key)
            .ok_or_else(|| Error::NotSpecifiedError("Field type of key was not specified".to_string()))
            .cloned()
    }

    fn validate_new_key(&self, key_value: &Value, typed_key: &K) -> Result<(), Error> {
        if key_value.get_field_type() != K::get_field_type() {
            return Err(Error::TypeError("Mismatched field type".to_string()));
        }

        if self.records.contains_key(typed_key) {
            return Err(Error::AlreadyExistsError("Key already exists".to_string()));
        }

        Ok(())
    }

    fn convert_record(&self, intermediate: IntermediateRecord) -> Result<Record, Error> {
        let mut values = HashMap::new();

        for (field_name, field_type) in &self.fields {
            let intermediate_value = intermediate.values.get(field_name)
                .ok_or_else(|| Error::MissingFieldError(format!("Record missing required field '{}'", field_name)))?;

            let value = intermediate_value.to_value(field_type)?;
            values.insert(field_name.clone(), value);
        }

        for field_name in intermediate.values.keys() {
            if !self.fields.contains_key(field_name) {
                return Err(Error::NotExistError(format!("Unexpected field '{}'", field_name)));
            }
        }

        Ok(Record { values })
    }

    pub fn add_record(&mut self, record: IntermediateRecord) -> Result<(), Error> {
        let record = self.convert_record(record)?;

        let key_value = record.values.get(&self.key)
            .ok_or_else(|| Error::MissingFieldError("The record is missing the key value".to_string()))?;

        let typed_key = K::from_value(key_value)?;

        self.validate_new_key(key_value, &typed_key)?;

        self.records.insert(typed_key, record);

        Ok(())
    }

    pub fn delete_record(&mut self, record_key: &K) -> Result<(), Error> {
        _ = self.records.remove(record_key)
            .ok_or_else(|| Error::NotExistError(format!("Table missing a record with key '{}'", record_key)))?;

        Ok(())
    }
    
    pub fn to_slice(&self) -> TableSlice {
        let records = self.records.values().cloned().collect::<Vec<_>>();
        
        TableSlice::new(records)
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
