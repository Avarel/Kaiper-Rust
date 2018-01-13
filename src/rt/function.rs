use rt::Obj;
use vm::{VM, /*VMFrame*/};
use vm::err::RTErr;

pub struct NativeFunction {
    pub name: String,
    pub func: fn(Vec<Obj>) -> Result<Obj, RTErr>,
}

use std::fmt;
impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "def {}", self.name)
    }
}

impl NativeFunction {
    pub fn new(
        name: &str,
        func: fn(Vec<Obj>) -> Result<Obj, RTErr>,
    ) -> Self {
        NativeFunction {
            name: String::from(name),
            func: func,
        }
    }
}
