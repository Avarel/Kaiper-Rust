#[macro_use]
extern crate downcast_rs;
extern crate linefeed;

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
use vm::VM;

fn main() {
    let inst = vec![
        Inst::PushInt(1), 
        Inst::PushInt(2), 
        Inst::Add,
        // get_store("lol")
    ];

    match VM::new(inst).run() {
        Ok(ans) => {
            println!("Ans: {}", ans)
        }
        Err(msg) => {
            println!("{}", msg)
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

// use linefeed::{ReadResult, Reader};
// fn loop_read() {
//     let mut reader = Reader::new("kaiper").unwrap();
//     reader.set_prompt(">>> ");

//     loop {
//         if let Ok(ReadResult::Input(line)) = reader.read_line() {
//             match line.as_ref() {
//                 "quit" => {
//                     break
//                 }
//                 _ => {
//                     match lexer::tokenizer::Tokenizer::new(line.as_ref()).parse() {
//                         Ok(ans) => println!("{:?}", ans),
//                         Err(e) => println!("Err: {}", e),
//                     }
//                 }
//             }
//         }
//     }
// }