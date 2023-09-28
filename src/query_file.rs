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
    pub raw_contents: Vec<String>,
    pub file_type: QueryFileType,
}

impl QueryFile {
    pub fn create(path: String, raw_contents: Vec<String>) -> QueryFile {
        match Self::get_file_type(&raw_contents) {
            Ok(t) => QueryFile {
                path,
                raw_contents,
                file_type: t,
            },
            Err(_) => panic!("Header Type Not Found"),
        }
    }

    fn get_file_type(raw_contents: &Vec<String>) -> Result<QueryFileType, ImproperSignatureError> {
        let sig = &raw_contents[0];
        if sig.starts_with("::lib") {
            return Ok(QueryFileType::Lib);
        } else if sig.starts_with("::proc") {
            return Ok(QueryFileType::Proc);
        }
        Err(ImproperSignatureError)
    }
}
