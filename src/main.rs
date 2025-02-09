pub mod file_load;
pub mod query_file;
pub mod tags;
pub mod parsers;

use std::rc::{Rc, Weak};

use file_load::load_dir;
use parsers::{common::{file_parse_state::{FileParseState, ParsedFunc, ParsedStruct}, file_parser::{parse_next, NextTokenEvent}}, tokenizer::tokenizer::tokenize};
use tags::Tag;

fn main() {
    let r = load_dir("./source-scripts");

    let mut structs: Vec<ParsedStruct> = vec![];
    let mut funcs: Vec<ParsedFunc> = vec![];

    for qf in r {
        //translate strings to real tokens 
        //compute tokens to grammar

        let tokens_res = tokenize(qf.tokens);

        if let Ok(tokens) = tokens_res {
            let mut state = FileParseState::Unknown;

            
            let mut peek_op = Some(0); 
            let mut skip_op = Some(0);
            let mut i  = 0; 
    
            while i < tokens.len() {
                println!("Parsing at {0}", i);

                let mut end = i + 1; 
              
                if let Some(peek) = peek_op {
                    end += peek; 
                    println!("Peeking to {0}", end);
                }

                let t = &tokens[i..end];


                match parse_next(&mut state, t, i){
                    Ok((s, p, sk)) => {
                        state = s; 
                        peek_op = p; 
                        skip_op = sk;
                    },
                    Err(e) => {
                        println!("Error Parsing {0}: {1} at {2}", qf.path, e.0, e.1);
                        break;
                    }
                }

                if let Some(skip) = skip_op {
                    i += skip; 
                    println!("Skipping to {0}", i);
                }
                i+=1; 
            }
            println!("Done Parsing {0}", qf.path);

        

            match state {
                FileParseState::Unknown => todo!(),
                FileParseState::Library(mut library_parse_state) => {
                    structs.append(&mut library_parse_state.structs);
                    funcs.append(&mut library_parse_state.funcs);

                },
                FileParseState::Procedure => todo!(),
            }



        }
        else if let Err(t) = tokens_res{
            println!("Error Parsing {0}: {1} at {2}", qf.path, t.0, t.1);
        }

        for s in &structs[0..] {
            println!("{0}", s.name);
        }

    }
    
}
