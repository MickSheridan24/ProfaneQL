use crate::parsers::common::{parse_symbol::{parse_sym, ParseSymOutcome}, tag_parse_state::TagParseState, parse_error::ParseError, reader_state::ReaderState, arg_parse_state::ArgParseState};

use super::StructTagParseState;

pub fn parse_struct_sym(
    contents: &Vec<String>,
    reader: &ReaderState,
    struct_state: &StructTagParseState,
    s: &String,
    ) -> Result<TagParseState, ParseError> {

    let res = parse_sym(contents, reader, struct_state, s)?;

    match res {
        ParseSymOutcome::Next(r) => return Ok(TagParseState::Struct(
            reader.next_pos(),
            StructTagParseState::Sym(r.to_owned()),
        )),
        ParseSymOutcome::Escape(r, ch) => {
            if ch == '(' {
                return Ok(TagParseState::Struct(
                    reader.next_pos(),
                    StructTagParseState::Args(r.to_owned(),
                                            vec![],
                                            ArgParseState::ArgName("".to_owned())),
                ));
            }
            else {
                return Err(ParseError(reader.line(), reader.pos(), "ParseStructSym: Illegal Character"));
            }
        },
    }
}
