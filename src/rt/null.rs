use rt::obj::Obj;

pub struct Null;
impl Obj for Null {
    fn truth_value(&self) -> bool {
        false
    }
}

use std::fmt;
impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "null")
    }
}
