#[macro_use]
extern crate downcast_rs;
extern crate rustyline;

mod scope;
mod ast;
mod rt;
mod lexer;
mod vm;

#[macro_use]
mod macros;

use rt::obj::Obj;
use scope::Scope;
use vm::inst::Inst;
use vm::{VM, VMContext};

use std::rc::Rc;

fn main() {
    // let hello = 1 + 2
    // hello = hello + 7
    // yield 1
    // yield 2
    // hello

    // let inst = vec![
    //     Inst::PushInt(1), 
    //     Inst::PushInt(2), 
    //     Inst::Add,
    //     Inst::Store(String::from("hello")),
    //     Inst::Get(String::from("hello")),
    //     Inst::PushInt(7),
    //     Inst::Add,
    //     Inst::Store(String::from("hello")),
    //     Inst::PushInt(1),
    //     Inst::Yield,
    //     Inst::PushInt(2),
    //     Inst::Yield,
    //     Inst::Get(String::from("hello")),
    // ];

    //loop_read();

    let hello = kaiper_tokens! {
        let x = 1 + 2;
        pls
    };

    println!("{:?}", hello);

    // let counter = 0
    // while true {
    //    yield counter
    //    counter = counter + 1 
    // }
    // let inst = vec![
    //     Inst::PushInt(0),
    //     Inst::Store(String::from("counter")),
    //     Inst::Get(String::from("counter")),
    //     Inst::Yield,
    //     Inst::Get(String::from("counter")),
    //     Inst::PushInt(1),
    //     Inst::Add,
    //     Inst::Store(String::from("counter")),
    //     Inst::Jump(2),
    // ];
    let inst = vec![
        Inst::PushStr(String::from("hello ")),
        Inst::PushInt(5),
        Inst::Get(String::from("coolFunction")),
        Inst::Invoke(2),
    ];

    let mut vm = VM::new(inst);
    let mut cont = VMContext::default();

    use rt::function::NativeFunction;
    cont.heap.insert(String::from("coolFunction"), NativeFunction::new("coolFunction", |args| {
        for rc in args {
            println!("{}", rc);
        }
        Ok(Rc::new(String::from("YOU HAVE EXECUTED COOLFUNCTION")))
    }));

    for _ in 0..10 {
        match vm.run_context(&mut cont) {
            Ok(Some(ans)) => {
                println!("Ans: {}", ans)
            }
            Ok(None) => {
                println!("Execution finished");
                break
            }
            Err(msg) => {
                println!("{:?}", msg);
                break
            }
        }
    }
}

// fn get_store(id: &str) -> Inst {
//     let name = String::from(id);
//     Inst::Store(name.as_ref())
// }

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

use rustyline::Editor;
use rustyline::error::ReadlineError;

fn loop_read() {
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline(">>> ") {
            Ok(line) => {
                match lexer::tokenizer::Tokenizer::new(&line).parse() {
                    Ok(list) => {
                        println!("Tokens: {:?}", list)
                    }
                    Err(err) => {
                        println!("Lexer err: {}", err)
                    }
                }
                
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("{}", err);
                break
            }
        }
    }
}