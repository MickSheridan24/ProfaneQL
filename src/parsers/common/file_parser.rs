use std::string::ParseError;

use crate::parsers::tokenizer::tokenizer::{HeaderType, PunctuationType, QueryToken, ReservedType, TokenizeError, TokenizeErrorType};

use super::file_parse_state::{FileParseState, FuncParseState, LibraryItemParseState, LibraryParseState, ParsedStruct, StructParseState, TypedValue};

pub type NextTokenEvent = (FileParseState, Option<usize>, Option<usize>);

pub fn parse_next(state: &FileParseState, tokens: &[QueryToken]) -> Result<NextTokenEvent, TokenizeError>{
    
    match state{
        FileParseState::Unknown => {
            match &tokens[0] {
                QueryToken::Header(h) => match h {
                    HeaderType::Lib => Ok((FileParseState::Library(LibraryParseState::new()), None, None)),
                }, 
                _ => return  Err(TokenizeError(TokenizeErrorType::NoHeader))
            }
        },
        FileParseState::Library(library_parse_state) => {
            let (lib, peek, skip) = parse_library(library_parse_state, tokens)?;
            return Ok((FileParseState::Library(lib), peek, skip));
            
        },
        FileParseState::Procedure => todo!(),
    }
}


pub fn parse_library(state: LibraryParseState, tokens:&[QueryToken])-> Result<(LibraryParseState, Option<usize>, Option<usize>), TokenizeError>  {
    if state.current_item.is_none(){
        if let QueryToken::Reserved(r) = &tokens[0] {
            let curr = match r {
                ReservedType::Func => LibraryItemParseState::Func(FuncParseState::new()),
                ReservedType::Struct => LibraryItemParseState::Struct(StructParseState::Symbol),
            };

            return Ok((LibraryParseState {
                current_item: Some(curr), 
                structs: state.structs,
                funcs: state.funcs
            }, None, None));

        }
        return Err(TokenizeError(TokenizeErrorType::InvalidToken));
    }
    
    match state.current_item.unwrap() {
        LibraryItemParseState::Struct(struct_parse_state) => {
            let (res, peek, skip) = parse_struct(struct_parse_state, tokens)?;
            if let StructParseState::Complete(parsed) = res{
                let mut new_structs = state.structs;
                new_structs.push(parsed);
                return Ok((LibraryParseState{
                    current_item:None, 
                    structs: new_structs,
                    funcs: state.funcs

                }, None, None));
            }
            else {
                return Ok((LibraryParseState{
                    current_item: Some(LibraryItemParseState::Struct(res)), 
                    structs: state.structs,
                    funcs: state.funcs
                }, peek, skip));
            }
        },
        LibraryItemParseState::Func(func_parse_state) => todo!(),
    }
}



pub fn parse_struct(state: StructParseState, tokens: &[QueryToken]) -> Result<(StructParseState, Option<usize>, Option<usize>), TokenizeError>{
   
    if let StructParseState::Props(name, mut props) = state {
       if tokens.len() < 4 {
        return Err(TokenizeError(TokenizeErrorType::ParserError));
       }
       match (&tokens[0], &tokens[1], &tokens[2], &tokens[3]){
        (QueryToken::Plain(n), QueryToken::Punctuation(PunctuationType::Colon), QueryToken::DataType(t), QueryToken::Punctuation(PunctuationType::Comma)) => {
            props.push(TypedValue(n.clone(), *t));
            let new_state= StructParseState::Props(name, props);
            return Ok((new_state, Some(4), Some(3)));
        },
        (QueryToken::Plain(n), QueryToken::Punctuation(PunctuationType::Colon), QueryToken::DataType(t), QueryToken::Punctuation(PunctuationType::ParenEnd)) => {
            props.push(TypedValue(n.clone(), *t));
            let result = ParsedStruct{
                name,
                props
            };
            let new_state= StructParseState::Complete(result);
            return Ok((new_state, Some(4), Some(3)));
        },
        _ => return Err(TokenizeError(TokenizeErrorType::InvalidToken))
       }

    }
    else if let StructParseState::Init(name) = state {
        if let QueryToken::Punctuation(PunctuationType::ParenStart) = &tokens[0]{
            let new_state = StructParseState::Props(name, vec![]);
            return Ok((new_state, Some(4), None));
        }
    }
    else if let StructParseState::Symbol = state {
        if let QueryToken::Plain(s) = &tokens[0] {
            return Ok((StructParseState::Init(s.clone()), None, None));
        }
    }


    return Err(TokenizeError(TokenizeErrorType::InvalidToken));
}