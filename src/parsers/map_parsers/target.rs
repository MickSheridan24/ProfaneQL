use crate::parsers::common::{reader_state::ReaderState, tag_parse_state::TagParseState, parse_error::ParseError, parse_symbol::{parse_sym, ParseSymOutcome}};

use super::MapTagParseState;


pub fn parse_map_target(
    contents: &Vec<String>,
    reader: &ReaderState,
    map_state: &MapTagParseState,
    s: &String,
    t: &String
    ) -> Result<TagParseState, ParseError> {

    let res = parse_sym(contents, reader, map_state, t)?;

    match res {
        ParseSymOutcome::Next(r) => return Ok(TagParseState::Map(
            reader.next_pos(),
            MapTagParseState::Target(s.to_owned(), r),
        )),
        ParseSymOutcome::Escape(r, ch) => {
            if ch == '(' {
                return Ok(TagParseState::Map(
                    reader.next_pos(),
                    MapTagParseState::Args(s.to_owned(),t.to_owned(), vec![])
                ));
            }
            else {
                return Err(ParseError(reader.line(), reader.pos(), "ParseMapTarget: Illegal Character"));
            }
        },
    }
}
