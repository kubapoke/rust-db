use std::collections::HashMap;
use crate::database::key::DatabaseKey;
use crate::database::table::Table;

pub struct Database<K: DatabaseKey> {
    tables: HashMap<String, Table<K>>,
}

pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}