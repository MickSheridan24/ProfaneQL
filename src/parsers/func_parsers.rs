use crate::parsers::func_parsers::args::{ArgParseState, parse_func_args};
use crate::parsers::func_parsers::body::{BodyParseType, parse_func_body};
use crate::parsers::func_parsers::sym::parse_func_sym;
use crate::tags::{FuncTag, SqlType, Tag, VarcharSize};
use super::common::{ParseError, ReaderState, TagParseState};

pub mod args;
pub mod body;
pub mod sym;

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
    else if let FuncTagParseState::Args(s, mut a, p) = func_state {
        return parse_func_args(contents, reader, &func_state, s, a, p);
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
        return Ok(TagParseState::Complete(Tag::Func(tag)));
    }
    Err(ParseError)
}
