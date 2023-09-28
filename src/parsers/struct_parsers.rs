use crate::tags::SqlType;

pub enum StructTagParseState {
    Sym(String),
    Members(String, Vec<(String, SqlType)>),
}
