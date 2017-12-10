use ast::*;
use scope::Scope;
use kp_rt::obj::Obj;
use std::rc::Rc;

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

            Expr::Add(ref rhs, ref lhs) => {
                // TODO add more operators
                let right = self.visit(rhs, context)?;
                let left = self.visit(lhs, context)?;
                Ok(Rc::new(right.add(&left)?))
            }

            Expr::Block(ref vec) => {
                let mut context = context.sub_scope();
                vec.iter()
                    .map(|expr| self.visit(expr, &mut context))
                    .last()
                    .unwrap_or_else(|| Ok(Rc::new(Obj::Null))) // return last result
            }

            Expr::Declare(ref ident, ref expr) => {
                let value = self.visit(expr, context)?;
                context.insert(
                    ident.to_owned(),
                    Rc::try_unwrap(value).map_err(|_| "Internal error")?,
                );
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
