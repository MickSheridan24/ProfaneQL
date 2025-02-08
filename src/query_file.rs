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

        let bytes = raw_contents.as_bytes();

        let mut start = 0;

        for (i, &item)in bytes.iter().enumerate(){
            if item.is_ascii_whitespace() {
                tokens.push(raw_contents[start..i].to_string());
                start = i+1; 
            }
            else if item == b'(' || item == b')' || item == b'{' || item == b'}' || item == b','  {
                tokens.push(raw_contents[start..i].to_string());
                tokens.push(raw_contents[i..i+1].to_string());
                start = i +1 ; 
            }
        }


        QueryFile {
                path,
                tokens,
        }
        
    }
}
