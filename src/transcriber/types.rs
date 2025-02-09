use crate::parsers::{func_parse::types::ParsedFunc, struct_parse::types::ParsedStruct};

pub struct Library {
    pub structs: Vec<ParsedStruct>,
    pub funcs: Vec<ParsedFunc>
}