#[derive(Debug)]
pub enum Obj {
    Int(i32),
    Number(f64),
    Boolean(bool),
    String(String),
    Null
}

impl Obj {
    pub fn add(&self, other: &Obj) -> Obj {
        match *self {
            Obj::Int(i) => int_add(i, other),
            Obj::Number(n) => num_add(n, other),
            Obj::String(ref s) => str_add(s, other),
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

fn num_add(n: f64, other: &Obj) -> Obj {
    match *other {
        Obj::Int(o) => Obj::Number(n + o as f64),
        Obj::Number(o) => Obj::Number(n + o),
        _ => unimplemented!()
    }
}

fn str_add(s: &String, other: &Obj) -> Obj {
    match *other {
        Obj::String(ref o) => {
            let mut s = s.to_owned();
            s.push_str(o);
            Obj::String(s)
        }
        _ => unimplemented!()
    }
}