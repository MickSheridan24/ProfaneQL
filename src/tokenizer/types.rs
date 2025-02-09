use std::fmt;

use crate::parsers::common::DataType;

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

pub enum PunctuationType{
    ParenStart,
    ParenEnd,
    CurlyStart,
    CurlyEnd,
    Comma,
    Colon
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


#[derive(Debug)]
pub struct TokenizeError (pub TokenizeErrorType, pub usize);
