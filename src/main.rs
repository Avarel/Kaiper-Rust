mod scope;
mod interpreter;
mod ast;
mod kp_rt;

use kp_rt::obj::Obj;
use scope::Scope;
use ast::Expr;
use interpreter::Interpreter;

fn main() {
    let ast = Expr::Block(vec![
        Expr::Declare(
            String::from("hello"),
            Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))),
        ),
        Expr::Declare(
            String::from("hello"),
            Box::new(Expr::Add(
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
        Ok(ans) => println!("{}", ans),
        Err(e) => println!("{}", e),
    }
}
