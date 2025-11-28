use crate::database::record::Record;
use crate::database::value::Value;

#[derive(Clone, Debug)]
pub struct TableSlice {
    pub(crate) records: Vec<Record>,
}

impl TableSlice {
    pub fn new(records: Vec<Record>) -> Self {
        Self { records }
    }
}

#[derive(Clone, Debug)]
pub struct SelectRow {
    pub values: Vec<(String, Value)>,
}

impl SelectRow {
    pub fn to_string(&self) -> String {
        self.values
            .iter()
            .map(|(field, value)| format!("{}: {}", field, value))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[derive(Clone, Debug)]
pub struct SelectResult {
    pub rows: Vec<SelectRow>,
}

impl SelectResult {
    pub fn to_string(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
