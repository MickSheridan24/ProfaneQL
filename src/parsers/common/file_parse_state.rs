


pub enum FileParseState {
    Unknown,
    Library(LibraryParseState),
    Procedure

}


pub struct  LibraryParseState {
    pub current_item: Option<LibraryItemParseState>,

    pub structs: Vec<ParsedStruct>,
    pub funcs: Vec<ParsedFunc>,

}

impl LibraryParseState {
    pub fn new() -> LibraryParseState{
        LibraryParseState {
            current_item: None, 
            structs: vec![],
            funcs: vec![], 
        }
    }

    pub fn set_curr(&mut self, curr: LibraryItemParseState){
        self.current_item = Some(curr); 
    }
}

pub enum LibraryItemParseState {
    Struct(StructParseState),
    Func(FuncParseState)
}


pub enum StructParseState {
    Symbol,
    Init(String),
    Props(String, Vec<TypedValue>),
    Complete(ParsedStruct)

}



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
pub struct ParsedStruct {
    pub name: String, 
    pub props: Vec<TypedValue>
}


#[derive(Clone)]
pub struct ParsedFunc {
    pub name: String ,
    pub args: Vec<TypedValue>,
    pub body: String
}

#[derive(Clone)]
pub struct TypedValue (
    pub String, 
    pub  DataType
);

#[derive(Clone, Copy)]
pub enum DataType {
    NVarchar,
    Int 
}