use crate::{parsers::{file_parse::{ parse_next, types::FileParseState}, func_parse::types::ParsedFunc, struct_parse::types::ParsedStruct}, query_file::QueryFile, tokenizer::{tokenize, types::{QueryToken, TokenizeError}}, transcriber::types::Library};

pub fn process_files(files: Vec<QueryFile>) -> Library{

    
    let mut structs: Vec<ParsedStruct> = vec![];
    let mut funcs: Vec<ParsedFunc> = vec![];

    for file in files {
        let path = &file.path; 

       match process_file(&file.tokens){
        Ok((mut s, mut f)) => {
            structs.append(&mut s);
            funcs.append(&mut f);
        },
        Err(e) => {
            println!("Error processing {0}: {1} at {2}", path.to_owned(), e.0, e.1);
        }
       }
    }

    Library {
        structs,
        funcs
    }
    
}
pub fn process_file(tokens: &Vec<String>) -> Result<(Vec<ParsedStruct>, Vec<ParsedFunc>), TokenizeError>{
    match tokenize(tokens){
        Ok(tokens) => {
            let mut state = FileParseState::Unknown;

        
        let mut peek_op = Some(0); 
        let mut skip_op = Some(0);
        let mut i  = 0; 

        while i < tokens.len() {

            let mut end = i + 1; 
          
            if let Some(peek) = peek_op {
                end += peek; 
            }

            let t = &tokens[i..end];


            match parse_next(&mut state, t, i){
                Ok((s, p, sk)) => {
                    state = s; 
                    peek_op = p; 
                    skip_op = sk;
                },
                Err(e) => {
                    return Err(e);
                }
            }

            if let Some(skip) = skip_op {
                i += skip; 
            }
            i+=1; 
        }

    

        match state {
            FileParseState::Unknown => todo!(),
            FileParseState::Library(mut library_parse_state) => {
                return Ok((library_parse_state.structs, library_parse_state.funcs));

            },
            FileParseState::Procedure => todo!(),
        }
        },
        Err(e) => Err(e),
    }

}


pub fn process_token(){

}