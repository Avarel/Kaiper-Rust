use kp_rt::obj::Obj;

pub struct Null;
impl Obj for Null {}

use std::fmt;
impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "null")
    }
}
