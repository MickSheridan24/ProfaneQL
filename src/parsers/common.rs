use std::{error::Error, fmt::Display};

use crate::tags::{StructTagParseState, Tag};

use super::func_parsers::FuncTagParseState;

#[derive(Debug)]
pub struct ParseError(pub usize, pub usize, pub &'static str);

impl Display for ParseError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "Unable to Read File Type"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct ReaderState(pub usize, pub usize);
impl ReaderState {
    pub fn line(&self) -> usize {
        self.0
    }
    pub fn pos(&self) -> usize {
        self.1
    }

    pub fn curr(&self, contents: Vec<String>, range: Option<usize>) -> String {
        let r = match range {
            None => 1,
            Some(c) => c,
        };
        let r = contents[self.line()][self.pos()..self.pos() + r].to_owned().clone();
        println!("{}",r);
        r
    }
    pub fn is_line_end(&self, contents: &Vec<String>) -> bool {
        !self.is_doc_end(contents) && self.pos() >= contents[self.line()].len()
    }

    pub fn has_left(&self, contents: Vec<String>, range: usize) -> bool{
        contents[self.line()].len() >= range
    }

    pub fn is_doc_end(&self, contents: &Vec<String>) -> bool {
        self.line() >= contents.len()
    }

    pub fn next_pos(&self) -> ReaderState {
        ReaderState(self.0, self.1 + 1)
    }
    pub fn next_line(&self) -> ReaderState {
        ReaderState(self.0 + 1, 0)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum TagParseState {
    None(ReaderState),
    Func(ReaderState, FuncTagParseState),
    Struct(ReaderState, StructTagParseState),
    Map(ReaderState),
    Complete(ReaderState, Tag),
    EndOfFile
}

