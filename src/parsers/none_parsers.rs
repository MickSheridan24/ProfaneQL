use super::{common::{tag_parse_state::TagParseState, reader_state::ReaderState, parse_error::ParseError}, struct_parsers::StructTagParseState, func_parsers::FuncTagParseState, map_parsers::MapTagParseState};


pub fn parse_none(contents: &Vec<String>, reader: &ReaderState) -> Result<TagParseState, ParseError> {
    if reader.is_doc_end(contents) {
        //End of file
        return Ok(TagParseState::EndOfFile);
    }

    if reader.is_line_end(contents) {
        //End of line
        return Ok(TagParseState::None(reader.next_line()));
    }

    let c = reader.curr((*contents).clone(), Option::None);
    if c == ":" {
        if reader.curr((*contents).clone(), Some(8)) == ":struct " {
            return Ok(TagParseState::Struct(
                ReaderState(reader.line(), reader.pos() + 8),
                StructTagParseState::Sym("".to_string()),
            ));
        } else if reader.curr((*contents).clone(), Some(6)) == ":func " {
            return Ok(TagParseState::Func(
                ReaderState(reader.line(), reader.pos() + 5),
                FuncTagParseState::Sym("".to_string()),
            ));
        } else if reader.curr((*contents).clone(), Some(5)) == ":map " {
            return Ok(TagParseState::Map(ReaderState(
                reader.line(),
                reader.pos() + 4),
                MapTagParseState::Source("".to_string())
            ));
        }
    } else if c.trim() == "" {
        return Ok(TagParseState::None(reader.next_pos()));
    }

    Err(ParseError(reader.line(), reader.pos(), "ParseNone: Illegal Character"))
}
