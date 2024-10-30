use crate::parsers::common::arg_parse_state::ArgParseState;
use crate::parsers::common::parse_error::ParseError;
use crate::parsers::common::reader_state::ReaderState;
use crate::parsers::common::tag_parse_state::TagParseState;
use crate::tags::{SqlType, VarcharSize};

use super::StructTagParseState;



pub fn parse_struct_args(
    contents: &Vec<String>,
    reader: &ReaderState,
    func_state: &StructTagParseState,
    s: &String,
    a: Vec<(String, SqlType)>,
    p: &ArgParseState,
) -> Result<TagParseState, ParseError> {

    if reader.is_line_end(&contents) {
        //next line
        return Ok(TagParseState::Struct(
            reader.next_line(),
            StructTagParseState::Args(s.to_owned(), a,
                                    ArgParseState::None),
        ));
    }

    let c = reader.curr((*contents).clone(), None);

    return match p {
        ArgParseState::None => parse_struct_args_none(c.as_str(), reader, func_state, &a, s),
        ArgParseState::Limbo(esc, next) => parse_func_args_limbo(c.as_str(), reader, func_state, s, &a, esc, next),
        ArgParseState::ArgName(n) => parse_struct_args_name(c.as_str(), reader, s, &a, n),
        ArgParseState::ArgType(n, t) => parse_struct_args_type(c.as_str(), reader,  s, &a,  n, t),
        ArgParseState::TryComplete((n, t)) => parse_func_args_try_complete(reader, s, &a,  n, t)
    }
}
fn parse_func_args_try_complete(reader: &ReaderState,
                                s: &String,
                                a: &Vec<(String, SqlType)>,
                                n: &String,
                                t: &String) -> Result<TagParseState, ParseError>{
    match parse_type(reader.line(), reader.pos(),  t) {
        Ok(tt) => {
            let mut ac = (*a).clone();
            ac.push((n.to_owned(), tt));
            return Ok(TagParseState::Struct(reader.next_pos(),
                                          StructTagParseState::Complete(s.to_owned(),ac)));
        },
        Err(e) => return Err(e),
    }
}

fn parse_struct_args_none(c: &str,
                        reader: &ReaderState,
                        func_state: &StructTagParseState,
                        a: &Vec<(String, SqlType)>,
                        s: &String,
) -> Result<TagParseState, ParseError> {
    if c.trim() == "" {
        return Ok(TagParseState::Struct(reader.next_pos(), (*func_state).clone()));
    }
    if c == ")" {
        return Ok(TagParseState::Struct(
            reader.next_pos(),
            StructTagParseState::Complete(s.to_owned(), (*a).clone()),
        ));
    }

    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            return Ok(TagParseState::Struct(reader.next_pos(),
                                          StructTagParseState::Args(s.to_owned(),
                                                                  (*a).clone(),
                                                                  ArgParseState::ArgName(c.to_owned()))));
        }
    }
    return Err(ParseError(reader.line(), reader.pos(), "ParseArgs-None: Expected ')' or whitespace"));
}

fn parse_func_args_limbo(c: &str,
                         reader: &ReaderState,
                         func_state: &StructTagParseState,
                         s: &String,
                         a: &Vec<(String, SqlType)>,
                         esc: &Vec<String>, next: &Box<ArgParseState>
) -> Result<TagParseState, ParseError> {

    if c.trim() == ""{
        return Ok(TagParseState::Struct(reader.next_pos(), (*func_state).clone()));
    }
    if esc.contains(&c.to_string()) {
        return Ok(TagParseState::Struct(
            reader.next_pos(),
            StructTagParseState::Args(s.to_owned(), (*a).clone(), (**next).clone()),
        ));
    }
    Err(ParseError(reader.line(), reader.pos(), "ParseArgs-Limbo: Expected whitespace or escape char"))
}

fn parse_struct_args_name(c: &str,
                        reader: &ReaderState,
                        s: &String,
                        a: &Vec<(String, SqlType)>,
                        n: &String) -> Result<TagParseState, ParseError> {
    if c.trim() == "" {
        return Ok(TagParseState::Struct(
            reader.next_pos(),
            StructTagParseState::Args(
                s.to_owned(),
                (*a).clone(),
                ArgParseState::Limbo(
                    vec![":".to_string()],
                    Box::new(ArgParseState::ArgType(n.to_owned(), "".to_string())),
                ),
            ),
        ))
    }
    if c == ":"{
        return Ok(TagParseState::Struct(
            reader.next_pos(),
            StructTagParseState::Args(
                s.to_owned(),
                (*a).clone(),
                ArgParseState::ArgType(n.to_owned(), "".to_string())
            ),
        ))
    }

    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            return Ok(TagParseState::Struct(reader.next_pos(),
                                          StructTagParseState::Args(s.to_owned(),
                                                                  (*a).clone(),
                                                                  ArgParseState::ArgName(n.to_owned() + c))));
        }
    }

    return Err(ParseError(reader.line(), reader.pos(), "ParseArgs-Name: Illegal character"))
}

fn parse_struct_args_type(c: &str,
                        reader: &ReaderState,
                        s: &String,
                        a: &Vec<(String, SqlType)>,
                        n: &String,
                        t: &String) -> Result<TagParseState, ParseError> {

    if c.trim() == ""{
        if t.to_owned() == ""{
            return Ok(
                TagParseState::Struct(
                    reader.next_pos(),
                    StructTagParseState::Args(s.to_owned(),
                                            (*a).clone(),
                                            ArgParseState::ArgType(n.to_owned(), "".to_owned()))));
        }

        return Ok(TagParseState::Struct(
            reader.next_pos(),
            StructTagParseState::Args(
                s.to_owned(),
                (*a).clone(),
                ArgParseState::Limbo(
                    vec![",".to_string(), ")".to_string()],
                    Box::new(ArgParseState::TryComplete((n.to_owned(), t.to_owned()))),
                ),
            ),
        ));
    }
    if c == ")" {
        match parse_type(reader.line(), reader.pos(),  t) {
            Ok(tt) => {
                let mut ac = (*a).clone();
                ac.push((n.to_owned(), tt));
                return Ok(TagParseState::Struct(reader.next_pos(),
                                      StructTagParseState::Complete(s.to_owned(), ac)));
            },
            Err(e) => return Err(e),
        }
    }

    if c == "," {
        match parse_type(reader.line(), reader.pos(), t) {
            Ok(tt) => {
                let mut ac = (*a).clone();
                ac.push((n.to_owned(), tt));
                return Ok(TagParseState::Struct(reader.next_pos(),
                              StructTagParseState::Args(s.to_owned(),
                                                      ac,
                                                      ArgParseState::None)))
            },
            Err(e) => return Err(e),
        }
    }

    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            return Ok(
                TagParseState::Struct(
                    reader.next_pos(),
                      StructTagParseState::Args(s.to_owned(),
                                   (*a).clone(),
                                          ArgParseState::ArgType(n.to_owned(), t.to_owned() + c))));
}
    }

    return Err(ParseError(reader.line(), reader.pos(), "ParseArgs-Type"))
}

fn parse_type(l: usize, pos: usize, t: &String) -> Result<SqlType, ParseError> {
    let tt = t.to_lowercase();
    if tt == "int"{
        return Ok(SqlType::Int);
    }
    if tt == "bool"{
        return Ok(SqlType::Bit)
    }
    if tt.starts_with("varchar"){
        return Ok(SqlType::String(VarcharSize::Max))
    }
    if tt == "datetime"{
        return Ok(SqlType::DateTime)
    }
    if tt == "decimal"{
        return Ok(SqlType::Float)
    }
    if tt == "tinyint"{
        return Ok(SqlType::TinyInt)
    }

    return Err(ParseError(l, pos, "ParseSqlType"))

}

