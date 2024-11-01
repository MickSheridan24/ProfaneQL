use crate::{parsers::{func_parsers::FuncTagParseState, struct_parsers::StructTagParseState, map_parsers::MapTagParseState}, tags::Tag};

use super::reader_state::ReaderState;



#[derive(Clone, Eq, PartialEq)]
pub enum TagParseState {
    None(ReaderState),
    Func(ReaderState, FuncTagParseState),
    Struct(ReaderState, StructTagParseState),
    Map(ReaderState, MapTagParseState),
    Complete(ReaderState, Tag),
    EndOfFile
}

