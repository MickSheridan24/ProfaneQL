use std::{fmt::Error, vec};

pub enum QueryToken {
    Header(HeaderType),
    Symbol,
    DataType,
    Punctuation,
    Values

}

pub enum HeaderType {
    Lib
}


pub enum TokenizeErrorType {
    EmptyToken,
    InvalidHeader
}
pub struct TokenizeError (TokenizeErrorType);



fn get_header(s: &str) -> Result<QueryToken, TokenizeError>{
    match s {
        "::lib" => Ok(QueryToken::Header(HeaderType::Lib)),
        _ => Err(TokenizeError(TokenizeErrorType::InvalidHeader))
    }

}


fn tokenize_string(string_token: &String)-> Result<QueryToken, TokenizeError>{
    if string_token.starts_with("::") {
       return get_header(&string_token);
    }
    Err(TokenizeError(TokenizeErrorType::EmptyToken))
}

pub fn tokenize(string_tokens: Vec<String>) -> Vec<Result<QueryToken, TokenizeError>> {
    return string_tokens.iter().map(|str| tokenize_string(str)).collect()
}


