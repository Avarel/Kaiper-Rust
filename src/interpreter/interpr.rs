use ast::*;
use scope::Scope;
use rt::obj::Obj;
use rt::null::Null;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use interpreter::err::IntrprErr;

pub struct Interpreter;
impl Interpreter {
    pub fn visit(
        &mut self,
        expr: &Expr,
        context: &Scope<String, Box<Obj>>,
    ) -> Result<Rc<Box<Obj>>, IntrprErr> {
        match *expr {
            Expr::String(ref s) => Ok(Rc::new(Box::new(s.to_owned()))),
            Expr::Int(i) => Ok(Rc::new(Box::new(i))),
            Expr::Number(n) => Ok(Rc::new(Box::new(n))),
            Expr::Boolean(b) => Ok(Rc::new(Box::new(b))),
            Expr::Null => Ok(Rc::new(Box::new(Null))),

            Expr::Identifier(ref ident) => match context.get(ident) {
                Some(i) => Ok(i),
                None => Err(IntrprErr::UndefinedVariable(ident.to_owned())),
            },

            Expr::BinaryOp(ref op, ref rhs, ref lhs) => {
                let right = &*self.visit(rhs, context)?;
                let left = &*self.visit(lhs, context)?;

                Ok(Rc::new(match *op {
                    BinaryOp::Add => right.add(left.borrow()),
                    BinaryOp::Sub => right.sub(left.borrow()),
                    BinaryOp::Mul => right.mul(left.borrow()),
                    BinaryOp::Div => right.div(left.borrow()),
                    _ => return Err(IntrprErr::Unimplemented),
                }.map_err(|m| IntrprErr::RuntimeErr(m))?))
            }

            Expr::Block(ref expr) => self.visit(expr, &mut context.sub_scope()),

            Expr::Stmts(ref vec) => {
                let mut last: Rc<Box<Obj>> = Rc::new(Box::new(Null));
                for expr in vec {
                    last = self.visit(expr, context)?;
                }
                Ok(last)
            }

            Expr::Declare(ref ident, ref expr) => {
                if context.map_contains(ident) {
                    return Err(IntrprErr::DuplicateVariable(ident.to_owned()));
                }

                let value = self.visit(expr, context)?;
                context.insert(
                    ident.to_owned(),
                    Rc::try_unwrap(value).map_err(|_| IntrprErr::Internal)?,
                );
                Ok(Rc::new(Box::new(Null)))
            }

            Expr::Assign(ref ident, ref expr) => {
                if !context.any_contains(ident) {
                    return Err(IntrprErr::DuplicateVariable(ident.to_owned()));
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

                Ok(Rc::new(Box::new(Null)))
            }

            _ => Err(IntrprErr::Unimplemented),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }
}
