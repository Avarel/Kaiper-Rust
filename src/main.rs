#[macro_use]
extern crate downcast_rs;

mod scope;
mod interpreter;
mod ast;
mod rt;
mod lexer;

#[macro_use]
mod macros;

use rt::obj::Obj;
use scope::Scope;
use ast::{BinaryOp, Expr};
use interpreter::interpr::Interpreter;

fn main() {
    // This is basically:
    //
    // let hello = 1 + 2
    // hello = hello + 3
    // return hello

    // match lexer::tokenizer::Tokenizer::new("let hello = 1 + 3.2e1").parse() {
    //     Ok(ans) => println!("{:?}", ans),
    //     Err(e) => println!("Err: {}", e),
    // }

    let ast = stmts! {
        expr!(let hello = expr!(Expr::Int(1), BinaryOp::Add, Expr::Int(2)));
        expr!(hello = expr!(Expr::Int(3), BinaryOp::Add, expr!(hello)));
        expr!(hello)
    };

    println!("{:?}", ast);

    let mut scope = Scope::<String, Box<Obj>>::new();
    let result = Interpreter::new().visit(&ast, &mut scope);

    match result {
        Ok(ans) => println!("Ans: {}", ans),
        Err(e) => println!("Err: {:?}", e),
    }
}
