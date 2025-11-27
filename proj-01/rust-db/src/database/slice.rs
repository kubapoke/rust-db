use crate::database::record::Record;

pub struct TableSlice {
    pub(crate) records: Vec<Record>,
}

impl TableSlice {
    pub fn new(records: Vec<Record>) -> Self {
        Self { records }
    }
}