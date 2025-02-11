use types::{FuncParseState, FuncToken, ParsedFunc};

use crate::tokenizer::types::{PunctuationType, QueryToken, TokenizeError, TokenizeErrorType};

use super::common::TypedValue;



pub mod types;



pub fn parse_func(state: &mut FuncParseState, tokens: &[QueryToken], index: usize) -> Result<(FuncParseState, Option<usize>, Option<usize>), TokenizeError>{

    match state {
        FuncParseState::Symbol => {
            if let QueryToken::Plain(s) = &tokens[0] {
                return Ok((FuncParseState::Init(s.clone()), None, None));
            }
            return Err(TokenizeError(TokenizeErrorType::InvalidToken("Expected struct name".into()), index))

        },
        FuncParseState::Init(name) => {
            if let QueryToken::Punctuation(PunctuationType::ParenStart) = &tokens[0]{
                let new_state = FuncParseState::Args(name.to_string(), vec![].into());
                return Ok((new_state, Some(3), None));
            }
            return Err(TokenizeError(TokenizeErrorType::InvalidToken("Expected \"(\"".into()), index))

        },
        FuncParseState::Args(name, args) => {
            if tokens.len() != 4 {
                return Err(TokenizeError(TokenizeErrorType::ParserError("Expected 4 Tokens".into()), index));
               }
               match (&tokens[0], &tokens[1], &tokens[2], &tokens[3]){
                (QueryToken::Plain(n), QueryToken::Punctuation(PunctuationType::Colon), QueryToken::DataType(t), QueryToken::Punctuation(PunctuationType::Comma)) => {
                    args.push(TypedValue(n.clone(), *t));
                    let new_state= FuncParseState::Args(name.to_string(), args.to_owned());
                    return Ok((new_state, Some(3), Some(3)));
                },
                (QueryToken::Plain(n), QueryToken::Punctuation(PunctuationType::Colon), QueryToken::DataType(t), QueryToken::Punctuation(PunctuationType::ParenEnd)) => {
                    args.push(TypedValue(n.clone(), *t));
                    
                    let new_state= FuncParseState::InitBody(name.to_owned(), args.to_owned());
                    return Ok((new_state, None, Some(3)));
                },
                _ => {
                    let got: Vec<String> =  tokens.into_iter().map(|t| t.to_string()).collect();

                    let err = "Expected property, got ".to_owned() + &got.join(", ");

                    return Err(TokenizeError(TokenizeErrorType::InvalidToken(err), index));

               }
            }
        },
        FuncParseState::InitBody(name, args) => {
            if let QueryToken::Punctuation(PunctuationType::CurlyStart) = &tokens[0]{
                let new_state = FuncParseState::Body(name.to_string(), args.to_owned(), vec![].into());
                return Ok((new_state, Some(3), None));
            }
            return Err(TokenizeError(TokenizeErrorType::InvalidToken("Expected \"(\"".into()), index))

        },
        FuncParseState::Body(name, args, func_tokens) => {
            match &tokens[0]{
                QueryToken::Symbol(s) => {
                    func_tokens.push(FuncToken::Symbol(s.to_owned()));
                    return Ok((FuncParseState::Body(name.to_owned(), args.to_owned(), func_tokens.to_owned()), None, None));
                },
                QueryToken::Plain(s) => {
                    func_tokens.push(FuncToken::Plain(s.to_owned()));
                    return Ok((FuncParseState::Body(name.to_owned(), args.to_owned(), func_tokens.to_owned()), None, None));
                },
                QueryToken::Punctuation(PunctuationType::CurlyEnd) => {
                    let final_tokens = func_tokens.to_owned();
                    let result = ParsedFunc{
                        name: name.to_string(),
                        args: args.to_owned(),
                        sections: final_tokens
                    };
                    let new_state= FuncParseState::Complete(result);
                    return Ok((new_state, None, None))
                },
                QueryToken::Punctuation(other) => {
                    func_tokens.push(FuncToken::Plain(other.to_string()));
                    return Ok((FuncParseState::Body(name.to_owned(), args.to_owned(), func_tokens.to_owned()), None, None));
                }
                _ => Err(TokenizeError(TokenizeErrorType::InvalidToken("Parse function body, expected Symbol or Plain".to_owned()), index))
            }
        },

        FuncParseState::Complete(_) => Err(TokenizeError(TokenizeErrorType::ParserError("Didn't process Completed Struct".to_owned()), index)),
    }


}