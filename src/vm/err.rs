use std::io::Error;

#[derive(Debug)]
pub enum VMErr {
    UnknownInstruction(u8),
    UnimplementedInstruction,
    Internal,
    IOErr(Error),
    UndefinedVariable(String),
    RtErr(String),
}