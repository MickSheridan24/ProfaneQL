use crate::parsers::common::{reader_state::ReaderState, tag_parse_state::TagParseState, parse_error::ParseError, parse_symbol::{parse_sym, ParseSymOutcome}};

use super::MapTagParseState;


pub fn parse_map_source(
    contents: &Vec<String>,
    reader: &ReaderState,
    map_state: &MapTagParseState,
    s: &String,
    ) -> Result<TagParseState, ParseError> {

    let res = parse_sym(contents, reader, map_state, s)?;

    match res {
        ParseSymOutcome::Next(r) => return Ok(TagParseState::Map(
            reader.next_pos(),
            MapTagParseState::Source(r.to_owned()),
        )),
        ParseSymOutcome::Escape(r, ch) => {
            if ch == ' ' {
                return Ok(TagParseState::Map(
                    reader.next_pos(),
                    MapTagParseState::Source(r.to_owned()),
                ));
            }
            else if ch == '#' {
                return Ok(TagParseState::Map(
                    reader.next_pos(),
                    MapTagParseState::Target(s.to_owned(),"".to_string())
                ));
            }
            else {
                return Err(ParseError(reader.line(), reader.pos(), "ParseMapSource: Illegal Character"));
            }
        },
    }
}
