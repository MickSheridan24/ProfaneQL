use std::{cell::RefCell, rc::Rc};

use crate::parsers::tokenizer::tokenizer::{HeaderType, PunctuationType, QueryToken, ReservedType, TokenizeError, TokenizeErrorType};

use super::file_parse_state::{FileParseState, FuncParseState, LibraryItemParseState, LibraryParseState, ParsedStruct, StructParseState, TypedValue};

pub type NextTokenEvent = (FileParseState, Option<usize>, Option<usize>);

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


pub fn parse_library(state:  &mut LibraryParseState, tokens:&[QueryToken], index:usize)-> Result<(LibraryParseState, Option<usize>, Option<usize>), TokenizeError>  {

    // let structs = state.structs.clone().get_mut();
    // let funcs = state.funcs.clone().get_mut(); 


    match &mut state.current_item {
        Some(s) => match s {
            LibraryItemParseState::Struct(struct_parse_state) => {
                let (res, peek, skip) = parse_struct(struct_parse_state, tokens, index)?;
                if let StructParseState::Complete(parsed) = res{
                    state.structs.push(parsed);
                    return Ok((LibraryParseState{
                        current_item:None.into(), 
                        structs: state.structs.clone(),
                        funcs:  state.funcs.clone()
                    }.into(), peek, skip));
                }
                else {
                    return Ok((LibraryParseState{
                        current_item: Some(LibraryItemParseState::Struct(res.into()).into()).into(), 
                        structs: state.structs.clone(),
                        funcs: state.funcs.clone()
                    }.into(), peek, skip));
                }
            },
            LibraryItemParseState::Func(func_parse_state) => todo!(),
        },
        None => {
            if let QueryToken::Reserved(r) = &tokens[0] {
                let curr = match r {
                    ReservedType::Func => LibraryItemParseState::Func(FuncParseState::new().into()),
                    ReservedType::Struct => LibraryItemParseState::Struct(StructParseState::Symbol.into()),
                }.into();
    
                return Ok((LibraryParseState {
                    current_item: Some(curr).into(), 
                    structs: state.structs.clone(),
                    funcs: state.funcs.clone()
                }.into(), None, None));
    
            }
            return Err(TokenizeError(TokenizeErrorType::InvalidToken("Expected \"struct\", or other declaration".into()), index));
        }
    }


}



pub fn parse_struct(state: &mut StructParseState, tokens: &[QueryToken], index: usize) -> Result<(StructParseState, Option<usize>, Option<usize>), TokenizeError>{

    match state {
        StructParseState::Symbol => {
            if let QueryToken::Plain(s) = &tokens[0] {
                return Ok((StructParseState::Init(s.clone()), None, None));
            }
            return Err(TokenizeError(TokenizeErrorType::InvalidToken("Expected struct name".into()), index))

        },
        StructParseState::Init(name) => {
            if let QueryToken::Punctuation(PunctuationType::ParenStart) = &tokens[0]{
                let new_state = StructParseState::Props(name.to_string(), vec![].into());
                return Ok((new_state, Some(3), None));
            }
            return Err(TokenizeError(TokenizeErrorType::InvalidToken("Expected \"(\"".into()), index))

        },
        StructParseState::Props(name, props) => {
            println!("{0}", tokens.len());
            if tokens.len() != 4 {
                return Err(TokenizeError(TokenizeErrorType::ParserError("Expected 4 Tokens".into()), index));
               }
               match (&tokens[0], &tokens[1], &tokens[2], &tokens[3]){
                (QueryToken::Plain(n), QueryToken::Punctuation(PunctuationType::Colon), QueryToken::DataType(t), QueryToken::Punctuation(PunctuationType::Comma)) => {
                    props.push(TypedValue(n.clone(), *t));
                    let new_state= StructParseState::Props(name.to_string(), props.to_owned());
                    return Ok((new_state, Some(3), Some(3)));
                },
                (QueryToken::Plain(n), QueryToken::Punctuation(PunctuationType::Colon), QueryToken::DataType(t), QueryToken::Punctuation(PunctuationType::ParenEnd)) => {
                    props.push(TypedValue(n.clone(), *t));
                    let final_props = props.to_owned();
                    let result = ParsedStruct{
                        name: name.to_string(),
                        props: final_props
                    };
                    let new_state= StructParseState::Complete(result);
                    return Ok((new_state, Some(3), Some(3)));
                },
                _ => {
                    let got: Vec<String> =  tokens.into_iter().map(|t| t.to_string()).collect();

                    let err = "Expected property, got ".to_owned() + &got.join(", ");

                    return Err(TokenizeError(TokenizeErrorType::InvalidToken(err), index));

               }
            }
        },
        StructParseState::Complete(parsed_struct) => todo!(),
    }


}