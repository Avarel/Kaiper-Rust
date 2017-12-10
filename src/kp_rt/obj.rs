pub enum Obj {
    Int(i32),
    Number(f64),
    Boolean(bool),
    String(String),
    // NativeFunction(Box<Fn() -> ()>),
    Null,
}

impl Obj {
    pub fn add(&self, other: &Obj) -> Result<Obj, String> {
        match *self {
            Obj::Int(i) => int_add(i, other),
            Obj::Number(n) => num_add(n, other),
            Obj::String(ref s) => str_add(s, other),
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

fn int_add(i: i32, other: &Obj) -> Result<Obj, String> {
    match *other {
        Obj::Int(o) => Ok(Obj::Int(i + o)),
        Obj::Number(o) => Ok(Obj::Number(i as f64 + o)),
        _ => Err(String::from("Type mismatch")),
    }
}

fn num_add(n: f64, other: &Obj) -> Result<Obj, String> {
    match *other {
        Obj::Int(o) => Ok(Obj::Number(n + o as f64)),
        Obj::Number(o) => Ok(Obj::Number(n + o)),
        _ => Err(String::from("Type mismatch")),
    }
}

fn str_add(s: &String, other: &Obj) -> Result<Obj, String> {
    let mut buf = s.to_owned();
    buf.push_str(&other.to_string());
    Ok(Obj::String(buf))
}
