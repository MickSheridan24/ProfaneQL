use crate::parsers::common::TypedValue;


pub enum FuncParseState {
    Symbol,
    Init(String),
    Args(String, Vec<TypedValue>),
    InitBody(String, Vec<TypedValue>),
    Body(String, Vec<TypedValue>, Vec<FuncToken>),
    Complete(ParsedFunc)

    
} 


#[derive(Clone)]
pub struct ParsedFunc {
    pub name: String ,
    pub args: Vec<TypedValue>,
    pub sections: Vec<FuncToken>
}


#[derive(Clone)]
pub enum FuncToken {
    Plain(String), 
    Symbol(String)
}
