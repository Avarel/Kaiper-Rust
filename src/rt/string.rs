use rt::obj::Obj;
use std::rc::Rc;
use vm::VM;

impl Obj for String {
    fn add(&self, other: Rc<Obj>, _: &mut VM) -> Result<Rc<Obj>, String> {
        let mut buf = self.to_owned();
        buf.push_str(&other.to_string());
        Ok(Rc::new(buf))
    }
}
