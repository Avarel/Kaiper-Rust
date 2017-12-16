use rt::obj::Obj;
use std::rc::Rc;

impl Obj for String {
    fn add(&self, other: &Obj) -> Result<Rc<Obj>, String> {
        let mut buf = self.to_owned();
        buf.push_str(&other.to_string());
        Ok(Rc::new(buf))
    }
}