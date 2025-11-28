use std::fmt::{Display, Formatter};
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

impl Display for SelectRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let values = self.values
            .iter()
            .map(|(field, value)| format!("{}: {}", field, value))
            .collect::<Vec<_>>()
            .join(", ");
        
        write!(f, "{}", values)
    }
}

#[derive(Clone, Debug)]
pub struct SelectResult {
    pub rows: Vec<SelectRow>,
}

impl Display for SelectResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rows = self.rows
            .iter()
            .map(|row| row.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        
        write!(f, "{}", rows)
    }
}
