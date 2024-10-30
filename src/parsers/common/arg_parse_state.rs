#[derive(PartialEq, Eq, Clone)]
pub enum ArgParseState {
    None,
    Limbo(Vec<String>, Box<ArgParseState>),
    ArgName(String),
    ArgType(String, String),
    TryComplete((String, String)),
}
