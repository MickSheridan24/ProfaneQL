

#[derive(Clone)]
pub struct TypedValue (
    pub String, 
    pub  DataType
);

#[derive(Clone, Copy)]
pub enum DataType {
    String,
    Int,
    Bool,
    Decimal, 
    TinyInt,
    DateTime
}