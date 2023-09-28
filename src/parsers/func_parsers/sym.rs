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
        return Err(ParseError);
    }
    let c = reader.curr(&contents, None);
    if c.trim() == "" {
        if s == "" {
            //whitespace leading
            return Ok(TagParseState::Func(reader.next_pos(), (*func_state).clone()));
        } else {
            //whitespace after sym
            return Err(ParseError);
        }
    }
    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            let m = s.to_owned() + c;
            return Ok(TagParseState::Func(
                reader.next_pos(),
                FuncTagParseState::Sym(m),
            ));
        } else if ch == '(' {
            if s == "" {
                //never wrote a name
                return Err(ParseError);
            } else {
                return Ok(TagParseState::Func(
                    reader.next_pos(),
                    FuncTagParseState::Args(s.to_owned(),
                                            vec![],
                                            ArgParseState::None),
                ));
            }
        }
    }
    return Err(ParseError);
}