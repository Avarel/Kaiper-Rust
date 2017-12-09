#[derive(Debug)]
pub enum Obj {
    Int(i32),
    Number(f64),
    Null
}

impl Obj {
    pub fn add(&self, other: &Obj) -> Obj {
        match *self {
            Obj::Int(i) => int_add(i, other),
            Obj::Number(n) => number_add(n, other),
            _ => unimplemented!()
        }
    }    
}

fn int_add(i: i32, other: &Obj) -> Obj {
    match *other {
        Obj::Int(o) => Obj::Int(i + o),
        Obj::Number(o) => Obj::Number(i as f64 + o),
        _ => unimplemented!()
    }
}

fn number_add(n: f64, other: &Obj) -> Obj {
    match *other {
        Obj::Int(o) => Obj::Number(n + o as f64),
        Obj::Number(o) => Obj::Number(n + o),
        _ => unimplemented!()
    }
}