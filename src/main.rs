#![allow(dead_code)]

#[macro_use]
extern crate downcast_rs;
extern crate rustyline;
extern crate byteorder;

mod scope;
mod rt;
mod lexer;
mod vm;
mod parser;

use vm::{VM, StackFrame};

use std::rc::Rc;

fn main() {
    // loop_read();
    // return;
    let string_pool: Vec<String> = vec!["hello there", "good bye", "WOOHOOOOOO", "one", "two", "printall"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut wtr = vm::inst_writer::InstWriter::new();
    wtr.load_str(0)
        .store(0, 3)
        .load_str(1)
        .store(0, 4)
        .get(5) // printall
        .load_str(2) // WOOHOO
        .get(3) // one
        .get(4) // two
        .invoke(3); // printall("WOOHOO", one, two)
    let code = wtr.complete();

    // TODO:
    // INSTRUCTIONS -> move to clike enums with bytes
    // inst -> 8 bits (1*u8)
    // Strings -> 24 bytes/192 bits (USE A CONSTANT TABLE)
    // int -> 4 byte/32 bit (4*u8)
    // num -> 8 byte/64 bit (8*u8)
    // variable names -> 8 byte/64 bit (8*u8) OR just get from string pool also
    // let inst = vec![
    //     Inst::LoadStr(0),
    //     Inst::Store(hstr("one"), 0),

    //     Inst::Get(hstr("one")),
    //     Inst::Get(hstr("println")),
    //     Inst::Invoke(1),

    //     Inst::PushTable,
    //     Inst::LoadStr(1),
    //     Inst::Store(hstr("one"), 0),
    //     Inst::Get(hstr("one")),
    //     Inst::Get(hstr("println")),
    //     Inst::Invoke(1),
    //     Inst::PopTable,

    //     Inst::PushTable,
    //     Inst::Get(hstr("one")),
    //     Inst::Get(hstr("println")),
    //     Inst::Invoke(1),
    //     Inst::PopTable,

    //     Inst::PushTable,
    //     Inst::LoadStr(2),
    //     Inst::Store(hstr("one"), 1),
    //     Inst::Get(hstr("one")),
    //     Inst::Get(hstr("println")),
    //     Inst::Invoke(1),
    //     Inst::PopTable,

    //     Inst::Get(hstr("one")),
    //     Inst::Get(hstr("println")),
    //     Inst::Invoke(1),
    // ];

    // /*
    // let one = "hello there";
    // println(one);
    // {
    //     let one = "good bye";
    //     println(one);
    // }
    // {
    //     println(one);
    // }
    // {
    //     one = "WOOOHOOOOOOO";
    //     println(one);
    // }
    // println(one);
    // */

    let mut vm = VM::new(code, string_pool);
    let mut cont = StackFrame::default();

    use rt::function::NativeFunction;
    cont.tables.insert(
        String::from("printall"),
        NativeFunction::new("printall", |args| {
            for rc in args {
                println!("{}", rc);
            }
            Ok(Rc::new(rt::null::Null))
        }),
    );

    for _ in 0..1 {
        match vm.run_context(&mut cont) {
            Ok(Some(ans)) => println!("Ans: {}", ans),
            Ok(None) => {
                println!("Execution finished");
                break;
            }
            Err(msg) => {
                println!("Error: {:?}", msg);
                break;
            }
        }
    }
}

pub fn hstr(string: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::default();
    string.to_owned().hash(&mut hasher);
    hasher.finish()
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
                match lexer::Tokenizer::new(&line).parse() {
                    Ok(list) => println!("Tokens: {:?}", list),
                    Err(err) => println!("Lexer err: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) |
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }
}
