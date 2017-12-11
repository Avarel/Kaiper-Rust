use kp_rt::obj::Obj;

pub fn add(n: f64, other: &Obj) -> Result<Obj, String> {
    match *other {
        Obj::Int(o) => Ok(Obj::Number(n + o as f64)),
        Obj::Number(o) => Ok(Obj::Number(n + o)),
        _ => Err(String::from("Type mismatch")),
    }
}