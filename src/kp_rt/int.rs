use kp_rt::obj::Obj;

// how to generify or do something about this redundancy

pub fn add(i: i32, other: &Obj) -> Result<Obj, String> {
    match *other {
        Obj::Int(o) => Ok(Obj::Int(i + o)),
        Obj::Number(o) => Ok(Obj::Number(i as f64 + o)),
        _ => Err(String::from("Type mismatch")),
    }
}

pub fn sub(i: i32, other: &Obj) -> Result<Obj, String> {
    match *other {
        Obj::Int(o) => Ok(Obj::Int(i - o)),
        Obj::Number(o) => Ok(Obj::Number(i as f64 - o)),
        _ => Err(String::from("Type mismatch")),
    }
}

pub fn mul(i: i32, other: &Obj) -> Result<Obj, String> {
    match *other {
        Obj::Int(o) => Ok(Obj::Int(i * o)),
        Obj::Number(o) => Ok(Obj::Number(i as f64 * o)),
        _ => Err(String::from("Type mismatch")),
    }
}

pub fn div(i: i32, other: &Obj) -> Result<Obj, String> {
    match *other {
        Obj::Int(o) => Ok(Obj::Int(i / o)),
        Obj::Number(o) => Ok(Obj::Number(i as f64 / o)),
        _ => Err(String::from("Type mismatch")),
    }
}