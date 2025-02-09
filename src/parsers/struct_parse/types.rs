use crate::parsers::common::TypedValue;



pub enum StructParseState {
    Symbol,
    Init(String),
    Props(String, Vec<TypedValue>),
    Complete(ParsedStruct)
}



#[derive(Clone)]
pub struct ParsedStruct {
    pub name: String, 
    pub props: Vec<TypedValue>
}
