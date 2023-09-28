use std::fs::{read, read_dir};
use crate::parsers::common::{ParseError, ReaderState, TagParseState};
use crate::parsers::func_parsers::args::ArgParseState;
use crate::parsers::func_parsers::FuncTagParseState;

pub fn parse_func_sym(
    contents: &Vec<String>,
    reader: &ReaderState,
    func_state: &FuncTagParseState,
    s: &String,
) -> Result<TagParseState, ParseError> {
    if reader.is_doc_end(&contents) || reader.is_line_end(&contents) {
        return Err(ParseError(reader.line(), reader.pos(), "ParseFuncSym: Doc Terminated Abruptly"));
    }
    let c = reader.curr((*contents).clone(), None);
    if c.trim() == "" {
        if s == "" {
            //whitespace leading
            return Ok(TagParseState::Func(reader.next_pos(), (*func_state).clone()));
        } else {
            //whitespace after sym
            return Err(ParseError(reader.line(), reader.pos(), "ParseFuncSym: Illegal Whitespace"));
        }
    }
    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            let m = s.to_owned() + c.as_str();
            return Ok(TagParseState::Func(
                reader.next_pos(),
                FuncTagParseState::Sym(m),
            ));
        } else if ch == '(' {
            if s == "" {
                //never wrote a name
                return Err(ParseError(reader.line(), reader.pos(), "ParseFuncSym: Symbol Not Given"));
            } else {
                return Ok(TagParseState::Func(
                    reader.next_pos(),
                    FuncTagParseState::Args(s.to_owned(),
                                            vec![],
                                            ArgParseState::ArgName("".to_owned())),
                ));
            }
        }
    }
    return Err(ParseError(reader.line(), reader.pos(), "ParseFuncSym: Illegal Character"));
}