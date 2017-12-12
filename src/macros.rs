macro_rules! stmts {
    () => (Expr::Null);
    ($($y: expr);*) => (Expr::Stmts(vec![$($y),*]))
}

macro_rules! expr {
    ($lhs: expr, $op: expr, $rhs: expr) => {
        Expr::BinaryOp(
            $op,
            Box::new($lhs),
            Box::new($rhs),
        )
    };
    (let $ident: ident = $x: expr) => {
        Expr::Declare(stringify!($ident).to_owned(), Box::new($x))
    };
    ($ident: ident = $x: expr) => {
        Expr::Assign(stringify!($ident).to_owned(), Box::new($x))
    };
    (return $x: expr) => {
        Expr::Return(Box::new($x));
    };
    (null) => {
        Expr::Null
    };
    ($ident: ident) => {
        Expr::Identifier(stringify!($ident).to_owned())
    };
}