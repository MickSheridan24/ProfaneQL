use std::{fmt::Error, vec};

use crate::parsers::common::file_parse_state::DataType;

pub enum QueryToken {
    Header(HeaderType),
    Reserved(ReservedType),
    Symbol(String),
    Punctuation(PunctuationType),
    DataType(DataType),
    Plain(String)

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
    InvalidToken,
    ParserError
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
pub struct TokenizeError (pub TokenizeErrorType);



fn get_header(s: &str) -> Result<QueryToken, TokenizeError>{
    match s {
        "::lib" => Ok(QueryToken::Header(HeaderType::Lib)),
        _ => Err(TokenizeError(TokenizeErrorType::InvalidHeader))
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
        _ => None
    }
}


fn tokenize_string(string_token: &String)-> Result<QueryToken, TokenizeError>{
    if string_token.starts_with("::") {
       return get_header(&string_token);
    }
    else if match_sym(&string_token).is_some() {
        return Ok(match_sym(&string_token).unwrap());
    }
    else if match_punc(&string_token).is_some() {
        return Ok(match_punc(&string_token).unwrap());
    }
    else if string_token.starts_with(":"){
        return Ok(QueryToken::Symbol(string_token[1..].to_string()))
    }
    else {
        return Ok(QueryToken::Plain(string_token.clone()))
    }
}

pub fn tokenize(string_tokens: Vec<String>) -> Result<Vec<QueryToken>, TokenizeError> {
    return string_tokens.iter().map(|str| tokenize_string(str)).collect()
}


