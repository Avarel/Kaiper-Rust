use ast::*;
use scope::Scope;
use kp_rt::obj::Obj;
use std::rc::Rc;

pub fn test() {
    let ast = Expr::Block(
        vec![
            Expr::Declare(String::from("hello"), Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(7))))),
            Expr::Identifier(String::from("hello"))
        ]
    );

    let mut scope = Scope::<String, Obj>::new();
    let result = Interpreter::new().visit(&ast, &mut scope);

    println!("{:?}", result)
}

pub struct Interpreter;
impl Interpreter {
    fn visit<'a>(&mut self, expr: &'a Expr, context: &mut Scope<String, Obj>) -> Rc<Obj> {
        match *expr {
            Expr::String(ref s) => Rc::new(Obj::String(s.to_owned())),
            Expr::Int(i) => Rc::new(Obj::Int(i)),
            Expr::Number(n) => Rc::new(Obj::Number(n)),
            Expr::Boolean(b) => Rc::new(Obj::Boolean(b)),
            Expr::Null => Rc::new(Obj::Null),
            Expr::Identifier(ref ident) => context.get(ident).unwrap(),
            Expr::Add(ref rhs, ref lhs) => { // TODO add more operators
                let right = self.visit(rhs, context);
                let left = self.visit(lhs, context);
                Rc::new(right.add(&left))
            }
            Expr::Block(ref vec) => vec.iter()
                .map(|expr| self.visit(expr, context))
                .last()
                .unwrap_or_else(|| Rc::new(Obj::Null)),
            Expr::Declare(ref ident, ref expr) => {
                let value = self.visit(expr, context);
                context.insert(ident.clone(), Rc::try_unwrap(value).unwrap());
                Rc::new(Obj::Null)
            }
            _ => unimplemented!(),
        }
    }
}

impl Interpreter {
    fn new() -> Self {
        Interpreter
    }
}
