use std::{fmt::{self, Error}, path::Display, vec};

pub mod types;
pub enum QueryToken {
    Header(HeaderType),
    Reserved(ReservedType),
    Symbol(String),
    Punctuation(PunctuationType),
    DataType(DataType),
    Plain(String)

}

impl QueryToken{
    pub fn to_string(&self)-> String{
        match self {
            
            QueryToken::Header(header_type) => "HEADER".to_owned(),
            QueryToken::Reserved(reserved_type) => "RESERVED".to_owned(),
            QueryToken::Symbol(_) => "SYM".to_owned(),
            QueryToken::Punctuation(punctuation_type) => "PUNC".to_owned(),
            QueryToken::DataType(data_type) => "TYPE".to_owned(),
            QueryToken::Plain(s) => format!("PLAIN(\"{0}\")", s).to_owned(),
        }
    }
}


pub enum HeaderType {
    Lib
}


pub enum ReservedType {
    Func, 
    Struct
}

#[derive(Debug)]
pub enum TokenizeErrorType {
    EmptyToken,
    InvalidHeader,
    NoHeader,
    InvalidToken(String),
    ParserError(String)
}

impl fmt::Display for TokenizeErrorType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenizeErrorType::EmptyToken => println!("Empty Token"),
            TokenizeErrorType::InvalidHeader => println!("Invalid Header"),
            TokenizeErrorType::NoHeader => println!("No Header"),
            TokenizeErrorType::InvalidToken(s) => println!("Invalid Token {0}", s),
            TokenizeErrorType::ParserError(s) => println!("Parser Error {0}", s),
        }
        Ok(())
    }
}

pub enum PunctuationType{
    ParenStart,
    ParenEnd,
    CurlyStart,
    CurlyEnd,
    Comma,
    Colon
}

#[derive(Debug)]
pub struct TokenizeError (pub TokenizeErrorType, pub usize);



fn get_header(s: &str) -> Result<QueryToken, TokenizeError>{
    println!("{0}", s.to_string());
    match s {
        "#lib" => Ok(QueryToken::Header(HeaderType::Lib)),
        _ => Err(TokenizeError(TokenizeErrorType::InvalidHeader, 0))
    }

}

fn match_sym(s: &str) -> Option<QueryToken> {
    match s {
        "func" => Some(QueryToken::Reserved(ReservedType::Func)),
        "struct" => Some(QueryToken::Reserved(ReservedType::Struct)),
        _ => None
    }
}

fn match_punc(s: &str) -> Option<QueryToken> {
    match s {
        "{" => Some(QueryToken::Punctuation(PunctuationType::CurlyStart)),
        "}" => Some(QueryToken::Punctuation(PunctuationType::CurlyEnd)),
        "(" => Some(QueryToken::Punctuation(PunctuationType::ParenStart)),
        ")" => Some(QueryToken::Punctuation(PunctuationType::ParenEnd)),
        "," => Some(QueryToken::Punctuation(PunctuationType::Comma)),
        ":" => Some(QueryToken::Punctuation(PunctuationType::Colon)),
        _ => None
    }
}

fn match_datatype(s: &str) -> Option<QueryToken> {
    match s {
        "int" => Some(QueryToken::DataType(DataType::Int)),
        "bool" => Some(QueryToken::DataType(DataType::Bool)),
        "datetime" => Some(QueryToken::DataType(DataType::DateTime)),
        "string" => Some(QueryToken::DataType(DataType::String)),
        "tinyint" => Some(QueryToken::DataType(DataType::TinyInt)),
        "decimal" => Some(QueryToken::DataType(DataType::Decimal)),
        _ => None
    }
}


fn tokenize_string(string_token: &String)-> Result<QueryToken, TokenizeError>{
    if string_token.starts_with("#") {
       return get_header(&string_token);
    }
    else if match_sym(&string_token).is_some() {
        return Ok(match_sym(&string_token).unwrap());
    }
    else if match_datatype(&string_token).is_some(){
        return Ok(match_datatype(&string_token).unwrap());
    }
    else if match_punc(&string_token).is_some() {
        return Ok(match_punc(&string_token).unwrap());
    }
    else if string_token.starts_with("*") && string_token.len() > 1{
        return Ok(QueryToken::Symbol(string_token[1..].to_string()))
    }
    else {
        return Ok(QueryToken::Plain(string_token.clone()))
    }
}

pub fn tokenize(string_tokens: Vec<String>) -> Result<Vec<QueryToken>, TokenizeError> {
    return string_tokens.iter().map(|str| tokenize_string(str)).collect()
}


