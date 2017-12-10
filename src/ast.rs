#[derive(Debug)]
pub enum Expr {
    Block(Vec<Expr>),
    String(String),
    Int(i32),
    Number(f64),
    Boolean(bool),
    Null,
    Add(Box<Expr>, Box<Expr>),
    Identifier(String),
    Declare(String, Box<Expr>),
}
