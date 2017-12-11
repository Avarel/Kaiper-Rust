mod scope;
mod interpreter;
mod ast;
mod kp_rt;

use kp_rt::obj::Obj;
use scope::Scope;
use ast::{BinaryOp, Expr};
use interpreter::Interpreter;

fn main() {

    // This is basically:
    //
    // let hello = 1 + 2
    // hello = hello + 3
    // return hello
    
    let ast = Expr::Stmts(vec![
        Expr::Declare(
            String::from("hello"),
            Box::new(Expr::BinaryOp(
                BinaryOp::Add,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2)),
            )),
        ),
        Expr::Assign(
            String::from("hello"),
            Box::new(Expr::BinaryOp(
                BinaryOp::Add,
                Box::new(Expr::Identifier(String::from("hello"))),
                Box::new(Expr::Int(3)),
            )),
        ),
        Expr::Identifier(String::from("hello")),
    ]);

    println!("{:?}", ast);

    let mut scope = Scope::<String, Obj>::new();
    let result = Interpreter::new().visit(&ast, &mut scope);

    match result {
        Ok(ans) => println!("Ans: {}", ans),
        Err(e) => println!("Err: {}", e),
    }
}
