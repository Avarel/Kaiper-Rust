#[derive(Debug)]
pub enum VMErr {
    UnknownInstruction,
    Internal,
    DefinedVariable,
    UndefinedVariable,
    RtErr(String)
}