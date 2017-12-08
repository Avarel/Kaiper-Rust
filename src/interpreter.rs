use ast::*;
use scope::Scope;
use kp_rt::obj::Obj;
use kp_rt::null::Null;

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
}

struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn visit_id(&mut self, expr: &Identifier, context: &mut Scope<&str, Box<Obj>>) -> Box<Obj> {
        Box::new(Null)
    }
}