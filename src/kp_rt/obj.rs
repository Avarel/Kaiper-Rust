use downcast_rs::*;
use kp_rt::*;

// pub enum Obj {
//     Int(i32),
//     Number(f64),
//     Boolean(bool),
//     String(String),
//     Null,
// }

// NativeFunction(Box<Fn() -> ()>),

use std::fmt;
pub trait Obj: fmt::Display + Downcast {
    fn add(&self, other: &Obj) -> Result<Box<Obj>, String> {
        Err(String::from("unimplemented"))
    }

    fn sub(&self, other: &Obj) -> Result<Box<Obj>, String> {
        Err(String::from("unimplemented"))
    }

    fn mul(&self, other: &Obj) -> Result<Box<Obj>, String> {
        Err(String::from("unimplemented"))
    }

    fn div(&self, other: &Obj) -> Result<Box<Obj>, String> {
        Err(String::from("unimplemented"))
    }
}
impl_downcast!(Obj);

impl Obj for bool {

}

pub struct Null;
impl Obj for Null {}

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "null")
    }
}