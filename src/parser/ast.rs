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
    Yield(Box<Expr>),

    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),

    // Operation
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),

    Invoke(Box<Expr>, Vec<Expr>),

    // Variables
    Ident(String),
    Let(String, Box<Expr>),
    Assign(String, Box<Expr>),

    ExternIdent(String),
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
