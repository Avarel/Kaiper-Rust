use std::io::Error;

#[derive(Debug)]
pub enum VMErr {
    UnknownInstruction,
    Internal,
    UnexpectedReadEOF,
    InterruptedRead,
    Test(Error),
    DefinedVariable,
    UndefinedVariable,
    RtErr(String),
}
