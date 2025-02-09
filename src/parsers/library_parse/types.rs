use crate::parsers::{func_parse::types::{FuncParseState, ParsedFunc}, struct_parse::types::{ParsedStruct, StructParseState}};


pub struct  LibraryParseState {
    pub current_item: Option<LibraryItemParseState>,

    pub structs: Vec<ParsedStruct>,
    pub funcs: Vec<ParsedFunc>,

}

impl LibraryParseState {
    pub fn new() -> LibraryParseState{
        LibraryParseState {
            current_item: None.into(), 
            structs: vec![],
            funcs: vec![]
        }
    }
}

pub enum LibraryItemParseState {
    Struct(StructParseState),
    Func(FuncParseState)
}

