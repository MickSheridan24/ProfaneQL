use types::{LibraryItemParseState, LibraryParseState};


use crate::tokenizer::types::{QueryToken, ReservedType, TokenizeError, TokenizeErrorType};

use super::{func_parse::{parse_func, types::FuncParseState}, struct_parse::{parse_struct, types::StructParseState}};


pub mod types;


pub fn parse_library(state:  &mut LibraryParseState, tokens:&[QueryToken], index:usize)-> Result<(LibraryParseState, Option<usize>, Option<usize>), TokenizeError>  {
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
            LibraryItemParseState::Func(func_parse_state) => {
                let (res, peek, skip) = parse_func(func_parse_state, tokens, index)?;
                if let FuncParseState::Complete(parsed) = res{
                    state.funcs.push(parsed);
                    return Ok((LibraryParseState{
                        current_item:None.into(), 
                        structs: state.structs.clone(),
                        funcs:  state.funcs.clone()
                    }.into(), peek, skip));
                }
                else {
                    return Ok((LibraryParseState{
                        current_item: Some(LibraryItemParseState::Func(res.into()).into()).into(), 
                        structs: state.structs.clone(),
                        funcs: state.funcs.clone()
                    }.into(), peek, skip));
                }
            },
        },
        None => {
            if let QueryToken::Reserved(r) = &tokens[0] {
                let curr = match r {
                    ReservedType::Func => LibraryItemParseState::Func(FuncParseState::Symbol.into()),
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

