pub mod file_load;
pub mod query_file;
pub mod tags;
pub mod parsers;

use file_load::load_dir;
use parsers::{common::{file_parse_state::FileParseState, file_parser::{parse_next, NextTokenEvent}}, tokenizer::tokenizer::tokenize};
use tags::Tag;

fn main() {
    let r = load_dir("./source-scripts");

    let mut tags: Vec<Tag> = vec![];

    for qf in r {
        //translate strings to real tokens 
        //compute tokens to grammar

        let tokens_res = tokenize(qf.tokens);

        if let Ok(tokens) = tokens_res {
            let mut state = FileParseState::Unknown;
            let peek_op = Some(0); 
            let skip_op = Some(0);
            let mut i  = 0; 
    
            while i < tokens.len() {
                state = FileParseState::Unknown;
               

                let mut end = i + 1; 
                if let Some(skip) = skip_op {
                    i += skip; 
                }
                if let Some(peek) = peek_op {
                    end += peek; 
                }

                let t = &tokens[i..end];

                if let Ok((s, p, sk)) = parse_next(&state, t){
                    
                }
                else {
                    println!("Error Parsing {0}", qf.path);
                    break;
                }
    
             
                
                i+=1; 
            }
        }
        else{
            println!("Error Parsing {0}", qf.path);
        }

    }
    for tag in tags{
        if let Tag::Func(t) = tag{
            let s = t.sym;
            println!("{}",s.as_str());
            println!("{}", t.args.len());
            println!("{}", t.body);
        }
        else if let Tag::Struct(t) = tag {
            let s = t.sym;
            println!("{}",s.as_str());
            println!("{}", t.members.len());
        }
    }
}
