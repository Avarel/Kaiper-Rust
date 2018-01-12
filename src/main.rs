#![allow(dead_code)]
#![feature(option_filter, refcell_replace_swap)]

#[macro_use]
extern crate downcast_rs;
extern crate rustyline;
extern crate byteorder;

mod rt;
mod lexer;
mod vm;
mod parser;

use vm::{VM};
use vm::frame::VMFrame;

use std::rc::Rc;

// use std::mem;
// unsafe fn get_erased_mut<'a, 'b>(vec: & mut Vec<u8>, start: usize, end: usize) -> &'b mut [u8] {
//     let ptr = vec.as_mut_ptr();
//     mem::transmute((ptr.offset(start as isize), end - start))
// }

fn main() {

    // loop_read();
    // return;
    // let string_pool: Vec<String> = vec!["hello there", "good bye", "WOOHOOOOOO", "one", "two", "printall"]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect();

    // use vm::alloc::Chunk;
    // let mut thing = [1,2,3];
    // {
    //     let mut chunk = Chunk::Heap(&mut thing);
    //     chunk.bytes_mut()[1] = 6;
    // }
    // println!("{:?}", thing);
    // let mut vec: Vec<u8> = vec![1,2,3];

    // let view = unsafe { get_erased_mut(&mut vec, 1, 3) };
    // let view2 = unsafe { get_erased_mut(&mut vec, 0, 2) };

    // println!("{:?}", view);

    // view[0] = 6;

    // println!("{:?}", view);
    // // println!("{:?}", vec);

    // // vec[1] = 5;

    // println!("{:?}", view);
    // println!("{:?}", view2);
    // println!("{:?}", vec);

    // let mut wtr = vm::inst_writer::InstWriter::new();
    // wtr.load_str(0)
    //     .store(0, 3)
    //     .load_str(1)
    //     .store(0, 4)
    //     .get(5) // printall
    //     .load_str(2) // WOOHOO
    //     .get(3) // one
    //     .get(4) // two
    //     .invoke(3); // printall("WOOHOO", one, two)
    // let code = wtr.complete();

    use parser::ast::Expr;
    let expr = Expr::Stmts(vec![
        Expr::ExternIdent(String::from("printall")),
        Expr::Invoke(
            Box::new(Expr::Ident(String::from("printall"))), 
            vec![Expr::If(
                Box::new(Expr::Boolean(true)),
                Box::new(Expr::Int(11231)), 
                Some(Box::new(Expr::Int(23439))),
            )]
        ),
        Expr::Let(
            String::from("hello"), 
            Box::new(Expr::String(String::from("hello there lol")))
        ),
        Expr::Block(Box::new(
            Expr::Stmts(vec![
                Expr::Invoke(
                    Box::new(Expr::Ident(String::from("printall"))), 
                    vec![Expr::Ident(String::from("hello"))]
                ),
                Expr::Assign(
                    String::from("hello"), 
                    Box::new(Expr::String(String::from("something completely different")))
                ),
                Expr::Invoke(
                    Box::new(Expr::Ident(String::from("printall"))), 
                    vec![Expr::Ident(String::from("hello"))]
                )
            ])
        )),
        Expr::Block(Box::new(
            Expr::Stmts(vec![
                Expr::Let(
                    String::from("hello"), 
                    Box::new(Expr::String(String::from("monkaMEGA")))
                ),
                Expr::Invoke(
                    Box::new(Expr::Ident(String::from("printall"))), 
                    vec![Expr::Ident(String::from("hello"))]
                )
            ])
        )),
        Expr::Invoke(
            Box::new(Expr::Ident(String::from("printall"))), 
            vec![Expr::Ident(String::from("hello"))]
        ),
        /*Expr::If(
            Box::new(Expr::Boolean(false)),
            Box::new(Expr::Invoke(
                Box::new(Expr::Identifier(String::from("printall"))), 
                vec![Expr::String(String::from("hello"))]
            )),
            Some(Box::new(
                Expr::If(
                    Box::new(Expr::Boolean(false)),
                    Box::new(Expr::Invoke(
                        Box::new(Expr::Identifier(String::from("printall"))), 
                        vec![Expr::String(String::from("WEW"))]
                    )),
                    Some(Box::new(Expr::Int(1234))),
                ),
            )),
        ),*/
    ]);

    let bytes = vm::compiler::Compiler::new().compile(&expr).unwrap();

    println!("Compilation complete!");

    // let expr = vec![

    // ]

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

    let mut vm = VM::load(bytes).unwrap();
    let mut cont = VMFrame::default();

    use rt::function::NativeFunction;
    cont.set_heap(
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
