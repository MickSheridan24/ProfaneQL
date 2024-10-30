

use crate::parsers::{func_parsers::FuncTagParseState, common::{reader_state::ReaderState, tag_parse_state::TagParseState,
    parse_error::ParseError, arg_parse_state::ArgParseState, parse_symbol::{parse_sym, ParseSymOutcome}}};

pub fn parse_func_sym(
    contents: &Vec<String>,
    reader: &ReaderState,
    func_state: &FuncTagParseState,
    s: &String,
    ) -> Result<TagParseState, ParseError> {

    let res = parse_sym(contents, reader, func_state, s)?;

    match res {
        ParseSymOutcome::Next(r) => return Ok(TagParseState::Func(
            reader.next_pos(),
            FuncTagParseState::Sym(r.to_owned()),
        )),
        ParseSymOutcome::Escape(r, ch) => {
            if ch == '(' {
                return Ok(TagParseState::Func(
                    reader.next_pos(),
                    FuncTagParseState::Args(s.to_owned(),
                                            vec![],
                                            ArgParseState::ArgName("".to_owned())),
                ));
            }
            else {
                let tag_parse_state = Err(ParseError(reader.line(), reader.pos(), "ParseFuncSym: Illegal Character"));
                return tag_parse_state;
            }
        },
    }
}
