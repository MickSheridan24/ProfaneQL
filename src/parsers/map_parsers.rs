use crate::tags::{MapTag, Tag};

use self::{source::parse_map_source, target::parse_map_target};

use super::common::{reader_state::ReaderState, tag_parse_state::TagParseState, parse_error::ParseError};
pub mod source;
pub mod target;


#[derive(Eq, PartialEq, Clone)]
pub enum MapTagParseState {
    Source(String),
    Target(String, String),
    Args(String, String, Vec<String>),
    Complete(String, String, Vec<String>)
}


pub fn parse_map(
    contents: &Vec<String>,
    reader: &ReaderState,
    map_state: &MapTagParseState,
) -> Result<TagParseState, ParseError> {
    if let MapTagParseState::Source(s) = map_state {
        return parse_map_source(contents, reader, map_state, s)
    }
    else if let MapTagParseState::Target(s, t) = map_state {
        return parse_map_target(contents, reader, map_state, s, t);
    }
    else if let MapTagParseState::Args(s, f, a) = map_state {
        todo!()
    }
    else if let MapTagParseState::Complete(s, f , a) = map_state {
        let tag = MapTag{
            strct: s.to_owned(),
            fnc: f.to_owned(),
            args: (*a).clone(),
        };
        return Ok(TagParseState::Complete(reader.next_pos(), Tag::Map(tag)));
    }
    Err(ParseError(reader.line(), reader.pos(), "ParseFunc"))
}