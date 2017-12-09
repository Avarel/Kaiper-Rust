use interpreter::Interpreter;
use std::rc::Rc;
use std::cell::RefCell;

// pub trait Expr {
//     fn visit(&self, visitor: Rc<RefCell<Visitor::Result>>, context: C) -> R;
//     // fn visit(&self, context: C) -> R;
//     // fn what(&self);
// }

// pub trait Visitor<R, C> {
//     type Result;
//     type Context;

//     fn visit_id(&mut self, expr: &Identifier, context: C) -> R;
//     fn visit_node(&mut self, expr: &Node, context: C) -> R;
// }

// pub struct Identifier {
//     pub name: String
// }

// impl Expr for Identifier {
//     fn visit<R, C>(&self, visitor: Rc<RefCell<Visitor<R, C>>>, context: C) -> R {
//         visitor.borrow_mut().visit_id(self, context)
//     }
// }

pub enum Expr {
    Block(Vec<Expr>),
    String(String),
    Int(i32),
    Number(f64),
    Boolean(bool),
    Null,
    Add(Box<Expr>, Box<Expr>),
    Identifier(String),
    Declare(String, Box<Expr>),
}


// pub enum Node {
//     String(String),
//     Int(i32),
//     Number(f64),
//     Boolean(bool),
//     Null,
// }

// // impl Expr for Node {
// //     fn visit<R, C>(&self, visitor: Rc<RefCell<Visitor<R, C>>>, context: C) -> R {
// //         visitor.borrow_mut().visit_node(self, context)
// //     }
// // }

// pub enum Op {
//     Add(Box<Expr>),
// }