use rt::obj::Obj;
use std::rc::Rc;
use vm::{VM, /*StackFrame*/};

pub struct NativeFunction {
    pub name: String,
    pub func: fn(Vec<Rc<Obj>>) -> Result<Rc<Obj>, String>,
}

use std::fmt;
impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "def {}", self.name)
    }
}

impl Obj for NativeFunction {
    fn invoke(&self, args: Vec<Rc<Obj>>, _: &mut VM) -> Result<Rc<Obj>, String> {
        return (self.func)(args);
    }
}

impl NativeFunction {
    pub fn new(
        name: &str,
        func: fn(Vec<Rc<Obj>>) -> Result<Rc<Obj>, String>,
    ) -> Self {
        NativeFunction {
            name: String::from(name),
            func: func,
        }
    }
}
