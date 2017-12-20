use rt::obj::Obj;
use std::rc::Rc;
use vm::{VM, StackFrame};

pub struct NativeFunction {
    pub name: String,
    pub func: Box<Fn(Vec<Rc<Obj>>) -> Result<Rc<Obj>, String>>
}

use std::fmt;
impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "def {}", self.name)
    }
}

impl Obj for NativeFunction {
    fn invoke(&self, args: Vec<Rc<Obj>>, _: &mut VM) -> Result<Rc<Obj>, String> {
        return (self.func)(args)
    }
}

impl NativeFunction {
    pub fn new<T: 'static + Fn(Vec<Rc<Obj>>) -> Result<Rc<Obj>, String>>(name: &str, func: T) -> Self {
        NativeFunction {
            name: String::from(name),
            func: Box::new(func)
        }
    }
}