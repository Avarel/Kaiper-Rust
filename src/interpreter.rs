use visitor::Visitor;
use ast::*;
use scope::Scope;

struct Interpreter;

// impl<'a> Visitor<i32, &'a Scope<'a, &'a str, i32>> for Interpreter {
//     fn visit_id(&mut self, expr: &Identifier, context: &'a Scope<'a, &'a str, i32>) -> i32 {
//         return 0;
//     }
// }
