#[macro_use]
extern crate downcast_rs;
extern crate linefeed;

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
    // loop_read();
}

// lets instead go for a stack-based VM
// explicit scoping instruction or implicit naming

// let hello = 1 + 2
// hello = hello + 3
// return hello
// loop_read();

// push 1
// push 2
// add (this pops 2 values and push the result)
// assign `hello` (pop a value and assigns to heap/scope)
// get `hello` (push value of `hello` to the stack)
// push 3
// add
// assign `hello`
// get `hello`
// return ### (pop and goto instruction ###)

// if true { println("Hello!") } else { println("No dice!") }

// 0 push true
// 1 if (successful go to #+1 else #+2)
// 2 goto 4
// 3 goto 8
// 4 push "Hello!"
// 5 get `println`
// 6 invoke 1 (pop 1 and pass it to arguments)
// 7 goto 11
// 8 push "No dice!"
// 9 get `println`
//10 invoke 1
//11 PROGRAM END

fn test_intrpr() {
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


use linefeed::{ReadResult, Reader};
fn loop_read() {
    let mut reader = Reader::new("kaiper").unwrap();
    reader.set_prompt(">>> ");

    loop {
        if let Ok(ReadResult::Input(line)) = reader.read_line() {
            match line.as_ref() {
                "quit" => {
                    break
                }
                _ => {
                    match lexer::tokenizer::Tokenizer::new(line.as_ref()).parse() {
                        Ok(ans) => println!("{:?}", ans),
                        Err(e) => println!("Err: {}", e),
                    }
                }
            }
        }
    }
}