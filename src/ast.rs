pub struct Identifier {
    pub name: String
}

pub enum Node {
    String(String),
    Int(i32),
    Number(f64),
    Boolean(bool),
    Null,
}