use crate::parsers::common::{ParseError, ReaderState, TagParseState};
use crate::parsers::func_parsers::body::BodyParseType;
use crate::parsers::func_parsers::FuncTagParseState;
use crate::tags::{SqlType, VarcharSize};

#[derive(PartialEq, Eq, Clone)]
pub enum ArgParseState {
    None,
    Limbo(Vec<String>, Box<ArgParseState>),
    ArgName(String),
    ArgType(String, String),
    TryComplete((String, String)),
}


pub fn parse_func_args(
    contents: &Vec<String>,
    reader: &ReaderState,
    func_state: &FuncTagParseState,
    s: &String,
    mut a: Vec<(String, SqlType)>,
    p: &ArgParseState,
) -> Result<TagParseState, ParseError> {
    // if reader.is_doc_end(&contents) {
    //     return Err(ParseError);
    // }
    if reader.is_line_end(&contents) {
        //next line
        return Ok(TagParseState::Func(
            reader.next_line(),
            FuncTagParseState::Args(s.to_owned(), a,
                                    ArgParseState::None),
        ));
    }

    let c = reader.curr(&contents, None);

    return match p {
        ArgParseState::None => parse_func_args_none(c, reader, func_state, s, a, &p),
        ArgParseState::Limbo(esc, next) => parse_func_args_limbo(c, reader, func_state, s, &a, &p, esc, next),
        ArgParseState::ArgName(n) => parse_func_args_name(c, reader, func_state, s, &a, &p, n),
        ArgParseState::ArgType(n, t) => parse_func_args_type(c, reader, func_state, s, &a, &p, n, t),
        ArgParseState::TryComplete(_) => todo!(),
    }
}

fn parse_func_args_none(c: &str,
                        reader: &ReaderState,
                        func_state: &FuncTagParseState,
                        s: &String,
                        mut a: Vec<(String, SqlType)>,
                        p: &ArgParseState) -> Result<TagParseState, ParseError> {
    if c.trim() == "" {
        return Ok(TagParseState::Func(reader.next_pos(), (*func_state).clone()));
    }
    if c == ")" {
        return Ok(TagParseState::Func(
            reader.next_pos(),
            FuncTagParseState::Body(s.to_owned(), a, BodyParseType::Unknown, "".to_string()),
        ));
    }
    return Err(ParseError);
}

fn parse_func_args_limbo(c: &str,
                         reader: &ReaderState,
                         func_state: &FuncTagParseState,
                         s: &String,
                         a: &Vec<(String, SqlType)>,
                         p: &ArgParseState, esc: &Vec<String>, next: &Box<ArgParseState>
) -> Result<TagParseState, ParseError> {

    if c.trim() == ""{
        return Ok(TagParseState::Func(reader.next_pos(), (*func_state).clone()));
    }
    if esc.contains(&c.to_string()) {
        return Ok(TagParseState::Func(
            reader.next_pos(),
            FuncTagParseState::Args(s.to_owned(), (*a).clone(), **next),
        ));
    }
    Err(ParseError)
}

fn parse_func_args_name(c: &str,
                        reader: &ReaderState,
                        func_state: &FuncTagParseState,
                        s: &String,
                        a: &Vec<(String, SqlType)>,
                        p: &ArgParseState,
                        n: &String) -> Result<TagParseState, ParseError> {

    if c.trim() == "" {
        return Ok(TagParseState::Func(
            reader.next_pos(),
            FuncTagParseState::Args(
                s.to_owned(),
                (*a).clone(),
                ArgParseState::Limbo(
                    vec![":".to_string()],
                    Box::new(ArgParseState::ArgType(n.to_owned(), "".to_string())),
                ),
            ),
        ))
    }

    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            return Ok(TagParseState::Func((reader.next_pos()),
                                          FuncTagParseState::Args(s.to_owned(),
                                                                  (*a).clone(),
                                                                  ArgParseState::ArgName(n.to_owned() + c))));
        }
    }

    return Err(ParseError)
}

fn parse_func_args_type(c: &str,
                        reader: &ReaderState,
                        func_state: &FuncTagParseState,
                        s: &String,
                        mut a: &Vec<(String, SqlType)>,
                        p: &ArgParseState,
                        n: &String,
                        t: &String) -> Result<TagParseState, ParseError> {

    if c.trim() == ""{
        return Ok(TagParseState::Func(
            reader.next_pos(),
            FuncTagParseState::Args(
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
        match parse_type(t) {
            Ok(tt) => {
                a.push((n.to_owned(), tt));
                return Ok(TagParseState::Func((reader.next_pos()),
                                      FuncTagParseState::Body(s.to_owned(),
                                                              (*a).clone(),
                                                              BodyParseType::Unknown,
                                                              "".to_string())));
            },
            Err(e) => return Err(e),
        }
    }

    if c == "," {
        match parse_type(t) {
            Ok(tt) => {
                a.push((n.to_owned(), tt));
                return Ok(TagParseState::Func((reader.next_pos()),
                              FuncTagParseState::Args(s.to_owned(),
                                                      (*a).clone(),
                                                      ArgParseState::None)))
            },
            Err(e) => return Err(e),
        }
    }

    if let Some(ch) = c.chars().nth(0) {
        if ch.is_alphabetic() || ch == '_' {
            return Ok(
                TagParseState::Func(
                    reader.next_pos(),
                      FuncTagParseState::Args(s.to_owned(),
                                   (*a).clone(),
                                          ArgParseState::ArgType(n.to_owned(), t.to_owned() + c))));
}
    }

    return Err(ParseError)
}

fn parse_type(t: &String) -> Result<SqlType, ParseError> {
    let tt = t.to_lowercase();
    if tt == "int"{
        return Ok(SqlType::Int);
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
    return Err(ParseError)

}

