use super::{reader_state::ReaderState, parse_error::ParseError};




pub enum ParseSymOutcome {
    Next(String),
    Escape(String, char)
}


pub fn parse_sym<T>(
    contents: &Vec<String>,
    reader: &ReaderState,
    state: T,
    s: &String,
) -> Result<ParseSymOutcome, ParseError> {
    if reader.is_doc_end(&contents) || reader.is_line_end(&contents) {
        return Err(ParseError(reader.line(), reader.pos(), "ParseSym: Doc Terminated Abruptly"));
    }
    let c = reader.curr((*contents).clone(), None);
    if c.trim() == "" {
        if s == "" {
            //whitespace leading
            return Ok(ParseSymOutcome::Next(s.to_owned()));
        }
    }
    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            let m = s.to_owned() + c.as_str();
            return Ok(ParseSymOutcome::Next(m.to_owned()));
        } else {
            if s == "" {
                //never wrote a name
                return Err(ParseError(reader.line(), reader.pos(), "ParseSym: Symbol Not Given"));
            } else {
                return Ok(ParseSymOutcome::Escape(s.to_owned(), ch));
            }
        }
    }
    return Err(ParseError(reader.line(), reader.pos(), "ParseSym: Illegal Character"));
}