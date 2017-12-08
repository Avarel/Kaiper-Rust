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
        // return context.
        Box::new(Null)
    }

    pub fn visit_node(&mut self, expr: &Node) -> Box<Obj> { // maybe return Rc<Obj> instead?
        match *expr {
            Node::String(ref s) => Box::new(s.clone()), // &'static str instead?
            Node::Int(i) => Box::new(i),
            Node::Number(n) => Box::new(n),
            Node::Boolean(b) => Box::new(b),
            Node::Null => Box::new(Null),
        }
    }
}