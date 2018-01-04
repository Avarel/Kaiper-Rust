use std::io::Error;

#[derive(Debug)]
pub enum VMErr {
    UnknownInstruction(u8),
    UnimplementedInstruction,
    Internal,
    IOErr(Error),
    UndefinedVariable(String),
    RTErr(RTErr),
}

#[derive(Debug)]
pub enum RTErr {
    TypeMismatch,
    Unimplemented
}

impl From<Error> for VMErr {
    fn from(e: Error) -> Self {
        VMErr::IOErr(e)
    }
}

impl From<RTErr> for VMErr {
    fn from(e: RTErr) -> Self {
        VMErr::RTErr(e)
    }
}
