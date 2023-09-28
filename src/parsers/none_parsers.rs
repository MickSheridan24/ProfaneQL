use crate::parsers::common::{ReaderState, TagParseState};
use crate::tags::{StructTagParseState};

use super::{common::ParseError, func_parsers::FuncTagParseState};

pub fn parse_none(contents: &Vec<String>, reader: &ReaderState) -> Result<TagParseState, ParseError> {
    if reader.is_doc_end(contents) {
        //End of file
        return Ok(TagParseState::EndOfFile);
    }

    if reader.is_line_end(contents) {
        //End of line
        return Ok(TagParseState::None(reader.next_line()));
    }

    let c = reader.curr(contents, Option::None);
    if c == ":" {
        if reader.curr(contents, Some(8)) == ":struct " {
            return Ok(TagParseState::Struct(
                ReaderState(reader.line(), reader.pos() + 8),
                StructTagParseState::Sym("".to_string()),
            ));
        } else if reader.curr(contents, Some(6)) == ":func " {
            return Ok(TagParseState::Func(
                ReaderState(reader.line(), reader.pos() + 5),
                FuncTagParseState::Sym("".to_string()),
            ));
        } else if reader.curr(contents, Some(5)) == ":map " {
            return Ok(TagParseState::Map(ReaderState(
                reader.line(),
                reader.pos() + 4,
            )));
        }
    } else if c.trim() == "" {
        return Ok(TagParseState::None(reader.next_pos()));
    }

    Err(ParseError)
}
