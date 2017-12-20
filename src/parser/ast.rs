#[derive(Debug)]
pub enum Expr {
    String(String),
    Int(i32),
    Number(f64),
    Boolean(bool),
    Null,

    // Flow control
    Block(Box<Expr>),
    Stmts(Vec<Expr>),
    Return(Box<Expr>),

    // Operation
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),

    // Variables
    Identifier(String),
    Let(String, Box<Expr>),
    Assign(String, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Neg,
    Pos,
}
