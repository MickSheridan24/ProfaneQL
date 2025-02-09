use types::{ParsedStruct, StructParseState};

use crate::tokenizer::types::{PunctuationType, QueryToken, TokenizeError, TokenizeErrorType};

use super::common::TypedValue;



pub mod types;



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
        StructParseState::Complete(_) => Err(TokenizeError(TokenizeErrorType::ParserError("Didn't process Completed Struct".to_owned()), index))
    }


}