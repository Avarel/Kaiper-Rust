pub enum InterpreterError {
    UndefinedVariable(String),
    DuplicateVariable(String),
    Unimplemented,
    Internal,
}