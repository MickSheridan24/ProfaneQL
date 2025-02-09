use crate::parsers::common::TypedValue;


pub struct FuncParseState {
    pub init: bool, 

    pub args: Vec<TypedValue>
}


impl FuncParseState {
    pub fn new() -> FuncParseState {
        return FuncParseState{
            init: false,
            args: vec![]
        }
    }
}


#[derive(Clone)]
pub struct ParsedFunc {
    pub name: String ,
    pub args: Vec<TypedValue>,
    pub body: String
}

