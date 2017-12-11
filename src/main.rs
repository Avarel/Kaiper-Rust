#![macro_use]
extern crate downcast_rs;

mod scope;
mod interpreter;
mod ast;
mod kp_rt;

use kp_rt::obj::Obj;
use scope::Scope;
use ast::{BinaryOp, Expr};
use interpreter::Interpreter;

macro_rules! stmts {
    () => (Expr::Null);
    ($($y: expr;)*) => (Expr::Stmts(vec![$($y),*]))
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
    ($ident: ident) => {
        Expr::Identifier(stringify!($ident).to_owned())
    };
}

fn main() {
    // This is basically:
    //
    // let hello = 1 + 2
    // hello = hello + 3
    // return hello

    let ast = stmts!{
        expr!(let hello = expr!(Expr::Int(1), BinaryOp::Add, Expr::Int(2)));
        expr!(hello = expr!(expr!(hello), BinaryOp::Add, Expr::Number(3.5)));
        expr!(hello);
    };

    println!("{:?}", ast);

    let mut scope = Scope::<String, Box<Obj>>::new();
    let result = Interpreter::new().visit(&ast, &mut scope);

    match result {
        Ok(ans) => println!("Ans: {}", ans),
        Err(e) => println!("Err: {}", e),
    }
}
