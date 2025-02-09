use types::{FileParseState, NextTokenEvent};

use crate::{parsers::library_parse::{parse_library, types::LibraryParseState}, tokenizer::types::{HeaderType, QueryToken, TokenizeError, TokenizeErrorType}, };

pub mod types;
pub fn parse_next(state: &mut FileParseState, tokens: &[QueryToken], index: usize) -> Result<NextTokenEvent, TokenizeError>{

    let got: Vec<String> =  tokens.into_iter().map(|t| t.to_string()).collect();

    let title = "Parsing  ".to_owned() + &got.join(", ");
    println!("{0}",title);

    match state {
        FileParseState::Unknown => {
            match &tokens[0] {
                QueryToken::Header(h) => match h {
                    HeaderType::Lib => Ok((FileParseState::Library(LibraryParseState::new()).into(), None, None)),
                }, 
                _ => return  Err(TokenizeError(TokenizeErrorType::NoHeader, index))
            }
        },
        FileParseState::Library(library_parse_state) => {
            let (lib, peek, skip) = parse_library(library_parse_state, tokens, index)?;
            return Ok((FileParseState::Library(lib).into(), peek, skip));
            
        },
        FileParseState::Procedure => todo!(),
    }

}




