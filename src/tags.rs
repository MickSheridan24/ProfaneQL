use crate::parsers::common::parse_error::ParseError;
use crate::parsers::common::reader_state::ReaderState;
use crate::parsers::common::tag_parse_state::TagParseState;
use crate::parsers::func_parsers::parse_func;
use crate::parsers::none_parsers::parse_none;
use crate::parsers::struct_parsers::parse_struct;

#[derive(PartialEq, Eq)]
pub enum QueryFileType {
    Lib,
    Proc,
}

#[derive(Eq, PartialEq, Clone)]
pub struct FuncTag {
    pub sym: String,
    pub args: Vec<(String, SqlType)>,
    pub body: String,
}
#[derive(PartialEq, Eq, Clone)]
pub struct StructTag {
    pub sym: String,
    pub members: Vec<(String, SqlType)>,
}


#[derive(PartialEq, Eq, Clone)]
pub enum VarcharSize {
    Max,
    Some(i32),
    None,
}
#[derive(PartialEq, Eq, Clone)]
pub enum SqlType {
    String(VarcharSize),
    Int,
    DateTime,
    Float,
    Bit,
    TinyInt
}

#[derive(Clone, Eq, PartialEq)]
pub enum Tag {
    Func(FuncTag),
    Struct(StructTag),
}


impl Tag {
    pub fn load_tags(contents: Vec<String>) -> Vec<Tag> {
        let mut parse_stage = TagParseState::None(ReaderState(1, 0));

        let mut tags = vec![];
        while parse_stage != TagParseState::EndOfFile {
            let next = Self::parse_next(&contents, &parse_stage);
            match next {
                Ok(r) => {
                    if let TagParseState::Complete(reader, t) = r.clone() {
                        tags.push(t);
                        parse_stage = TagParseState::None(reader)
                    }
                    else {
                        parse_stage = r.clone();
                    }
                }
                Err(e) => { panic!("Line {} Pos {} : {}", e.0, e.1, e.2); }
            }
        }
        return tags;
    }

    fn parse_next(
        contents: &Vec<String>,
        state: &TagParseState,
    ) -> Result<TagParseState, ParseError> {
        return match state {
            TagParseState::None(reader) =>
                parse_none(&contents, reader),
            TagParseState::Func(reader, func_state) =>
                parse_func( contents, reader, func_state),
            TagParseState::Struct(reader, struct_state) =>
                parse_struct(contents, reader, struct_state),
            TagParseState::Map(_) => todo!(),
            TagParseState::Complete(_, _) => Err(ParseError(0, 0, "Complete")),
            TagParseState::EndOfFile => Err(ParseError(0, 0, "End Of File"))
        }
    }
}
