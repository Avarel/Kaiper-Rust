
use kp_rt::*;

pub enum Obj {
    Int(i32),
    Number(f64),
    Boolean(bool),
    String(String),
    Null,
}
// NativeFunction(Box<Fn() -> ()>),

impl Obj { // Will use std::ops later
    pub fn add(&self, other: &Obj) -> Result<Obj, String> {
        match *self {
            Obj::Int(i) => int::add(i, other),
            Obj::Number(n) => num::add(n, other),
            Obj::String(ref s) => string::add(s, other),
            _ => Err(String::from("unimplemented")),
        }
    }

    pub fn sub(&self, other: &Obj) -> Result<Obj, String> {
        match *self {
            Obj::Int(i) => int::sub(i, other),
            _ => Err(String::from("unimplemented")),
        }
    }

    pub fn mul(&self, other: &Obj) -> Result<Obj, String> {
        match *self {
            Obj::Int(i) => int::mul(i, other),
            _ => Err(String::from("unimplemented")),
        }
    }

    pub fn div(&self, other: &Obj) -> Result<Obj, String> {
        match *self {
            Obj::Int(i) => int::div(i, other),
            _ => Err(String::from("unimplemented")),
        }
    }
}

use std::fmt;
impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Obj::Null => String::from("null"),
                Obj::Int(i) => i.to_string(),
                Obj::Number(n) => n.to_string(),
                Obj::Boolean(b) => b.to_string(),
                Obj::String(ref s) => s.to_owned(),
                _ => String::from("<unable to display>"),
            }
        )
    }
}
