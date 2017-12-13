#[derive(Debug)]
pub enum IntrprErr {
    UndefinedVariable(String),
    DuplicateVariable(String),
    Unimplemented,
    Internal,
    RuntimeErr(String),
}