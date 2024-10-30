use crate::parsers::func_parsers::body::{BodyParseType, parse_func_body};
use crate::tags::{FuncTag, SqlType, Tag};

use self::args::parse_func_args;
use self::sym::parse_func_sym;

use super::common::arg_parse_state::ArgParseState;
use super::common::parse_error::ParseError;
use super::common::reader_state::ReaderState;
use super::common::tag_parse_state::TagParseState;

pub mod args;
pub mod sym;
pub mod body;

#[derive(Eq, PartialEq, Clone)]
pub enum FuncTagParseState {
    Sym(String),
    Args(String,  Vec<(String, SqlType)>, ArgParseState),
    Body(String, Vec<(String, SqlType)>, BodyParseType, String),
    Complete(String, Vec<(String, SqlType)>, String)
}

pub fn parse_func(
    contents: &Vec<String>,
    reader: &ReaderState,
    func_state: &FuncTagParseState,
) -> Result<TagParseState, ParseError> {
    if let FuncTagParseState::Sym(s) = func_state {
        return parse_func_sym(contents, reader, func_state, s);
    }
    else if let FuncTagParseState::Args(s, a, p) = func_state {
        return parse_func_args(contents, reader, &func_state, s, (*a).clone(), p);
    }
    else if let FuncTagParseState::Body(s, a, t, b) = func_state {
        return parse_func_body(contents, reader, func_state, s, &a, &t, &b)
    }
    else if let FuncTagParseState::Complete(s, a , b) = func_state {
        let tag = FuncTag{
            sym: s.to_owned(),
            args: (*a).clone(),
            body: b.to_owned(),
        };
        return Ok(TagParseState::Complete(reader.next_pos(), Tag::Func(tag)));
    }
    Err(ParseError(reader.line(), reader.pos(), "ParseFunc"))
}
