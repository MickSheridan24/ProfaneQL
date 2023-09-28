use std::{fmt::Display};
use crate::parsers::common::{ParseError, ReaderState, TagParseState};
use crate::parsers::func_parsers::parse_func;
use crate::parsers::none_parsers::parse_none;

#[derive(PartialEq, Eq)]
pub enum QueryFileType {
    Lib,
    Proc,
}

#[derive(Eq, PartialEq, Clone)]
pub struct FuncTag {
    pub sym: String,
    pub args: Vec<(String, SqlType)>,
    pub body: String,
}
#[derive(PartialEq, Eq, Clone)]
pub struct StructTag {
    pub sym: String,
    pub members: Vec<(String, SqlType)>,
}

#[derive(PartialEq, Eq, Clone)]
pub enum StructTagParseState {
    Sym(String),
    Members(String, Vec<(String, SqlType)>),
}

#[derive(PartialEq, Eq, Clone)]
pub enum VarcharSize {
    Max,
    Some(i32),
    None,
}
#[derive(PartialEq, Eq, Clone)]
pub enum SqlType {
    String(VarcharSize),
    Int,
    DateTime,
    Float,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Tag {
    Func(FuncTag),
    Struct(StructTag),
}


impl Tag {
    pub fn load_tags(contents: Vec<String>) -> Vec<Tag> {
        let mut parse_stage = TagParseState::None(ReaderState(1, 0));

        let mut tags = vec![];
        while parse_stage != TagParseState::EndOfFile {
            let next = Self::parse_next(&contents, &parse_stage);
            if let Ok(r) = next{
                if let TagParseState::Complete(t) = r {
                    tags.push(t);
                }
                parse_stage = r.clone();
            }
            if let Err(ParseError) = next{
                panic!("Error Parsing File");
            }
        }
        return tags;
    }

    fn parse_next(
        contents: &Vec<String>,
        state: &TagParseState,
    ) -> Result<TagParseState, ParseError> {
        return match state {
            TagParseState::None(reader) =>
                parse_none(&contents, reader),
            TagParseState::Func(reader, func_state) =>
                parse_func( contents, reader, func_state),
            TagParseState::Struct(_, _) => todo!(),
            TagParseState::Map(_) => todo!(),
            TagParseState::Complete(res) => Err(ParseError),
            TagParseState::EndOfFile => Err(ParseError)
        }
    }
}
