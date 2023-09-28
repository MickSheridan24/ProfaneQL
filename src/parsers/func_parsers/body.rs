use crate::parsers::common::{ParseError, ReaderState, TagParseState};
use crate::parsers::func_parsers::FuncTagParseState;
use crate::tags::SqlType;

#[derive(PartialEq, Eq, Clone)]
pub enum BodyParseType{
    Unknown,
    Block,
    Expression
}



pub fn parse_func_body(
    contents: &Vec<String>,
    reader: &ReaderState,
    func_state: &FuncTagParseState,
    s: &String,
    a: &Vec<(String, SqlType)>,
    ty: &BodyParseType,
    b: &String
) -> Result<TagParseState, ParseError>
{
    if reader.is_doc_end(contents) {
        return Err(ParseError);
    }
    if reader.is_line_end(contents) {
        //next line
        return Ok(TagParseState::Func(
            reader.next_line(),
            (*func_state).clone()
        ));
    }

    let c = reader.curr(contents, None);

    if c == "{" {
        if b == "" && *ty == BodyParseType::Unknown{
            return Ok(TagParseState::Func(
                reader.next_pos(),
                FuncTagParseState::Body(s.to_owned(), (*a).clone(), BodyParseType::Block, b.to_owned())
            ))
        }
    }
    if c == "}"{
        if *ty == BodyParseType::Block{
            return Ok(TagParseState::Func(
                reader.next_pos(),
                FuncTagParseState::Complete(s.to_owned(), (*a).clone(), b.to_owned())
            ))
        }
    }
    if c == ";" {
        if *ty == BodyParseType::Expression{
            return Ok(TagParseState::Func(
                reader.next_pos(),
                FuncTagParseState::Complete(s.to_owned(), (*a).clone(), b.to_owned())
            ))
        }
        else if *ty == BodyParseType::Block{
            return Ok(TagParseState::Func(
                reader.next_pos(),
                FuncTagParseState::Body(s.to_owned(), (*a).clone(),
                                        BodyParseType::Block, b.to_owned() + c)
            ))
        }
    }
    if c.trim() == "" {
        if *ty == BodyParseType::Unknown{
            return Ok(TagParseState::Func(reader.next_pos(), (*func_state).clone()));
        }
        else {
            return Ok(TagParseState::Func(
                reader.next_pos(),
                FuncTagParseState::Body(s.to_owned(), (*a).clone(), (*ty).clone(), b.to_owned() + c)
            ))
        }
    }

    Err(ParseError)
}
