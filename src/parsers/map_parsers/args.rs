
use crate::parsers::common::{reader_state::ReaderState, tag_parse_state::TagParseState, parse_error::ParseError, parse_symbol::{parse_sym, ParseSymOutcome}};

use super::MapTagParseState;

pub fn parse_map_args(
    contents: &Vec<String>,
    reader: &ReaderState,
    map_state: &MapTagParseState,
    s: &String,
    t: &String,
    a: Vec<String>,
    curr: &String
) -> Result<TagParseState, ParseError> {

    let res = parse_sym(contents, reader, map_state, curr)?;

    match res {
        ParseSymOutcome::Next(r) => return Ok(TagParseState::Map(
            reader.next_pos(),
           MapTagParseState::Args(s.to_owned(), t.to_owned(), a, r.to_owned()),
        )),
        ParseSymOutcome::Escape(r, ch) => {
            if ch == ' ' {
                return Ok(TagParseState::Map(
                    reader.next_pos(),
                    MapTagParseState::Args(s.to_owned(), t.to_owned(), a, r.to_owned()),
                ));
            }
            else if ch == ',' {
                if curr.len() > 0 {
                    let mut ac = a;
                    ac.push(curr.to_owned());
                    return Ok(TagParseState::Map(
                        reader.next_pos(),
                        MapTagParseState::Args(s.to_owned(), t.to_owned(), ac, "".to_string())
                    ));
                }
                else {
                    return Err(ParseError(reader.line(), reader.pos(), "ParseMapArg: Empty Argument"));
                }

            }
            else if ch == ')' {
                if curr.len() > 0 {
                    let mut ac = a;
                    ac.push(curr.to_owned());
                    return Ok(TagParseState::Map(
                        reader.next_pos(),
                        MapTagParseState::Complete(s.to_owned(),t.to_owned(), ac)
                    ));
                }
                return Err(ParseError(reader.line(), reader.pos(), "ParseMapSource: Empty Arguments"));
            }
            else {
                return Err(ParseError(reader.line(), reader.pos(), "ParseMapSource: Illegal Character"));
            }
        },
    }

}