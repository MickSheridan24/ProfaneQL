use crate::parsers::library_parse::types::LibraryParseState;

pub type NextTokenEvent = (FileParseState, Option<usize>, Option<usize>);
pub enum FileParseState {
    Unknown,
    Library(LibraryParseState),
    Procedure

}

