use std::collections::HashMap;

trait DatabaseKey: Eq {}

impl<T: Eq> DatabaseKey for T {}

enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}

struct Record {
    values: HashMap<String, Value>,
}

struct Database<K: DatabaseKey> {
    records: HashMap<K, Record>,
}