use std::{
    error::Error,
    fmt::{Debug, Display},
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
    pub tokens: Vec<String>
}

impl QueryFile {
    pub fn create(path: String, raw_contents: String) -> QueryFile {
        let tokens = raw_contents.split_whitespace().map(|f| f.to_owned()).collect();

        QueryFile {
                path,
                tokens,
        }
        
    }
}
