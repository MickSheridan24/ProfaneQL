use std::{
    error::Error,
    fmt::{Debug, Display}, os::linux::raw,
};

#[derive(PartialEq, Eq)]
pub enum QueryFileType {
    Lib,
    Proc,
}

#[derive(Debug)]
pub struct ImproperSignatureError;

impl Display for ImproperSignatureError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl Error for ImproperSignatureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "Unable to Read File Type"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub struct QueryFile {
    pub path: String,
    pub tokens: Vec<String>,
}

impl QueryFile {
    pub fn create(path: String, raw_contents: String) -> QueryFile {

        let mut tokens: Vec<String> = vec![];


        let mut start = 0;

        for (i, item)in raw_contents.chars().enumerate(){
            print!("{0}", item);
            if item.is_ascii_whitespace() || item == '\n' || item == '\r' || item == '\t' || item.to_string().trim().len() == 0 {
                if raw_contents[start..i].trim().len() > 0 {

                    tokens.push(raw_contents[start..i].to_string());
                }
                start = i+1; 
            }
            else if item == '(' || item == ')' || item == '{' || item == '}' || item == ',' || item== ':'  {

                let curr = raw_contents[start..i].to_string();
                let nxt = raw_contents[i..i+1].to_string();

                println!("\nPUNC {0} ", curr.trim());
                println!("PUNC 2 {0}", nxt.trim());
                if curr.trim().len() > 0 {
                    tokens.push(curr.trim().to_owned());
                }
                
                tokens.push(nxt.trim().to_owned());
                start = i +1 ; 
            }
        }

        for token in &tokens[0..]{
            print!("[{0}] ", token);
        }

        QueryFile {
                path,
                tokens,
        }
        
    }
}
