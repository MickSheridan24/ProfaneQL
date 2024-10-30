use crate::tags::{SqlType, StructTag, Tag};

use self::{args::parse_struct_args, sym::parse_struct_sym};

use super::common::{arg_parse_state::ArgParseState, reader_state::ReaderState, tag_parse_state::TagParseState, parse_error::ParseError};

pub mod args;
pub mod sym;

#[derive(Eq, PartialEq, Clone)]
pub enum StructTagParseState {
    Sym(String),
    Args(String,  Vec<(String, SqlType)>, ArgParseState),
    Complete(String, Vec<(String, SqlType)>)
}


pub fn parse_struct(
    contents: &Vec<String>,
    reader: &ReaderState,
    struct_state: &StructTagParseState,
) -> Result<TagParseState, ParseError> {
    if let StructTagParseState::Sym(s) = struct_state {
        return parse_struct_sym(contents, reader, struct_state, s);
    }
    else if let StructTagParseState::Args(s, a, p) = struct_state {
        return parse_struct_args(contents, reader, &struct_state, s, (*a).clone(), p);
    }
    else if let StructTagParseState::Complete(s, a) = struct_state {
        let tag = StructTag{
            sym: s.to_owned(),
            members: (*a).clone(),
        };
        return Ok(TagParseState::Complete(reader.next_pos(), Tag::Struct(tag)));
    }
    Err(ParseError(reader.line(), reader.pos(), "ParseFunc"))
}