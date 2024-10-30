
use crate::{parsers::common::{reader_state::ReaderState, tag_parse_state::TagParseState, parse_error::ParseError, arg_parse_state::ArgParseState}};

use super::StructTagParseState;



pub fn parse_struct_sym(
    contents: &Vec<String>,
    reader: &ReaderState,
    struct_state: &StructTagParseState,
    s: &String,
) -> Result<TagParseState, ParseError> {
    if reader.is_doc_end(&contents) || reader.is_line_end(&contents) {
        return Err(ParseError(reader.line(), reader.pos(), "ParseStructSym: Doc Terminated Abruptly"));
    }
    let c = reader.curr((*contents).clone(), None);
    if c.trim() == "" {
        if s == "" {
            //whitespace leading
            return Ok(TagParseState::Struct(reader.next_pos(), (*struct_state).clone()));
        } else {
            //whitespace after sym
            return Err(ParseError(reader.line(), reader.pos(), "ParseStructSym: Illegal Whitespace"));
        }
    }
    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            let m = s.to_owned() + c.as_str();
            return Ok(TagParseState::Struct(
                reader.next_pos(),
                StructTagParseState::Sym(m),
            ));
        } else if ch == '(' {
            if s == "" {
                //never wrote a name
                return Err(ParseError(reader.line(), reader.pos(), "ParseStructSym: Symbol Not Given"));
            } else {
                return Ok(TagParseState::Struct(
                    reader.next_pos(),
                    StructTagParseState::Args(s.to_owned(),
                                            vec![],
                                            ArgParseState::ArgName("".to_owned())),
                ));
            }
        }
    }
    return Err(ParseError(reader.line(), reader.pos(), "ParseStructSym: Illegal Character"));
}