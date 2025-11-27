#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyType {
    String,
    Int,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldType {
    Bool,
    String,
    Int,
    Float,
}
