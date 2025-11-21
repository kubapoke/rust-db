use std::collections::HashMap;
use crate::database::key::DatabaseKey;
use crate::database::record::Record;
use crate::database::value::FieldType;

pub struct Table<K: DatabaseKey> {
    key: String,
    fields: HashMap<String, FieldType>,
    records: HashMap<K, Record>,
}