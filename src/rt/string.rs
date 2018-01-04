use rt::obj::Obj;
use std::rc::Rc;
use vm::VM;
use vm::err::RTErr;

impl Obj for String {
    fn add(&self, other: Rc<Obj>, _: &mut VM) -> Result<Rc<Obj>, RTErr> {
        let mut buf = self.to_owned();
        buf.push_str(&other.to_string());
        Ok(Rc::new(buf))
    }
}
