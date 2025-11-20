use std::collections::HashMap;

trait DatabaseKey: Eq + Ord {}

impl DatabaseKey for String {}
impl DatabaseKey for i64 {}

enum FieldType {
    Bool,
    String,
    Int,
    Float,
}

enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

struct Record {
    values: HashMap<String, Value>,
}

struct Table<K: DatabaseKey> {
    key: String,
    fields: HashMap<String, FieldType>,
    records: HashMap<K, Record>,
}

struct Database<K: DatabaseKey> {
    tables: HashMap<String, Table<K>>,
}

enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}