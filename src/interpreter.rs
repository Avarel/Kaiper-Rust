use ast::*;
use scope::Scope;
use kp_rt::obj::Obj;
use std::rc::Rc;

pub fn test() {
    let mut scope1 = Scope::<&str, &i32>::new();

    let mut obj_scope = Scope::<&str, Box<Obj>>::new();

    scope1.insert("wow", &213);

    let mut please = scope1.sub_scope();

    {
        please.insert("what", &23);

        declare(&mut scope1);

        // let borrow: &i32 = please.get(&"what").unwrap();
        // let borrow = please.get(&"what").unwrap();
        println!("plase[wow] {}", please.get(&"wow").unwrap());

        println!("scope[what] {}", scope1.get(&"wow").unwrap());

        please.insert("what", &23);

        println!("please[what] = {}", please.get(&"what").unwrap());

        println!("scope[what] = {}", scope1.get(&"what").unwrap());
    }
}

fn declare(scope: &mut Scope<&str, &i32>) {
    let mut sub_scope = scope.sub_scope();
    sub_scope.insert("what", &234);

    scope.insert("what", &10000);

    //println!("{:?}", Interpreter::new().visit(&Expr::Add(Box::new(Expr::Int(3)), Box::new(Expr::Int(4))), &mut Scope::new()))
}

pub struct Interpreter;
impl Interpreter {
    fn visit<'a>(&mut self, expr: &'a Expr, context: &mut Scope<&'a str, Obj>) -> Rc<Obj> {
        match *expr {
            // Expr::String(ref s) => Rc::new(s.to_owned()),
            Expr::Int(i) => Rc::new(Obj::Int(i)),
            Expr::Number(n) => Rc::new(Obj::Number(n)),
            // Expr::Boolean(b) => Rc::new(b),
            Expr::Null => Rc::new(Obj::Null),
            Expr::Identifier(ref string) => context.get(&&**string).unwrap(),
            Expr::Add(ref rhs, ref lhs) => {
                let right = self.visit(&*rhs, context);
                let left = self.visit(&*lhs, context);
                Rc::new(right.add(&left))
            },
            _ => unimplemented!(),
        }
    }
}

impl Interpreter {
    fn new() -> Self {
        Interpreter
    }
}
