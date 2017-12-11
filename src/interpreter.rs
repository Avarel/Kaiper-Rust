use ast::*;
use scope::Scope;
use kp_rt::obj::Obj;
use std::rc::Rc;
use std::cell::RefCell;
use std;

pub struct Interpreter;
impl Interpreter {
    pub fn visit(
        &mut self,
        expr: &Expr,
        context: &mut Scope<String, Obj>,
    ) -> Result<Rc<Obj>, String> {
        match *expr {
            Expr::String(ref s) => Ok(Rc::new(Obj::String(s.to_owned()))),
            Expr::Int(i) => Ok(Rc::new(Obj::Int(i))),
            Expr::Number(n) => Ok(Rc::new(Obj::Number(n))),
            Expr::Boolean(b) => Ok(Rc::new(Obj::Boolean(b))),
            Expr::Null => Ok(Rc::new(Obj::Null)),

            Expr::Identifier(ref ident) => match context.get(ident) {
                Some(i) => Ok(i),
                None => Err(format!("Undefined variable {}", ident)),
            },

            Expr::BinaryOp(ref op, ref rhs, ref lhs) => {
                // TODO add more operators
                let right = self.visit(rhs, context)?;
                let left = self.visit(lhs, context)?;
                Ok(Rc::new(match *op {
                    BinaryOp::Add => right.add(&left)?,
                    BinaryOp::Sub => right.sub(&left)?,
                    BinaryOp::Mul => right.mul(&left)?,
                    BinaryOp::Div => right.div(&left)?,
                    _ => return Err(format!("Unimplemented operation {:?}", op)),
                }))
            }

            Expr::Block(ref expr) => self.visit(expr, &mut context.sub_scope()),

            Expr::Stmts(ref vec) => {
                let mut last = Rc::new(Obj::Null);
                for expr in vec {
                    last = self.visit(expr, context)?;
                }
                Ok(last)
            }

            Expr::Declare(ref ident, ref expr) => {
                if context.map_contains(ident) {
                    return Err(format!("Variable {} is already defined", ident));
                }

                let value = self.visit(expr, context)?;
                context.insert(
                    ident.to_owned(),
                    Rc::try_unwrap(value).map_err(|_| "Internal error")?,
                );
                Ok(Rc::new(Obj::Null))
            }

            Expr::Assign(ref ident, ref expr) => {
                if !context.any_contains(ident) {
                    return Err(format!("Variable {} has not been declared", ident));
                }

                let value = self.visit(expr, context)?;
                context
                    .maps
                    .iter()
                    .rev()
                    .map(|rc| RefCell::borrow_mut(rc))
                    .find(|map| map.contains_key(ident))
                    .unwrap()
                    .insert(ident.to_owned(), value);

                Ok(Rc::new(Obj::Null))
            }

            _ => Err(String::from("Unimplemented instruction")),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }
}
