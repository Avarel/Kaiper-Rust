use std::io::Error;

#[derive(Debug)]
pub enum VMErr {
    UnknownInstruction(u8),
    UnimplementedInstruction,
    Internal,
    UnexpectedReadEOF,
    InterruptedRead,
    Test(Error),
    DefinedVariable,
    UndefinedVariable,
    RtErr(String),
}
