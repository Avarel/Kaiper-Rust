#[derive(Debug)]
pub enum Expr {
    Block(Box<Expr>),
    Stmts(Vec<Expr>),
    String(String),
    Int(i32),
    Number(f64),
    Boolean(bool),
    Null,
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
    Identifier(String),
    Declare(String, Box<Expr>),
    Assign(String, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
}
