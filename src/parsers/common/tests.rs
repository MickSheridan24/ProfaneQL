
#[cfg(test)]
mod tests {

    use crate::parsers::common::{parse_symbol::{parse_sym, ParseSymOutcome}, reader_state::ReaderState, parse_error::ParseError};

    fn test_script() -> Vec<String> {
        return vec!["Hello World!".to_owned()]
    }
    #[test]
    fn test_parse_sym_happy() -> Result<(), ParseError> {
        let res = parse_sym(&test_script(), &ReaderState(0, 6), "none", &"".to_owned())?;

        match res {
            ParseSymOutcome::Next(s) => assert_eq!(s, "W"),
            ParseSymOutcome::Escape(_, _) => assert!(false),
        };
        Ok(())
    }

    


}
