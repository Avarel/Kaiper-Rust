pub mod inst;

use vm::inst::Inst;
use scope::Scope;
use std::borrow::{Borrow, BorrowMut};
use rt::obj::Obj;
use rt::null::Null;
use std::rc::Rc;

pub struct VM {
    head: usize, // instruction index
    inst: Vec<Inst>,
    stack: Vec<Rc<Obj>>,
    heap: Scope,
}

macro_rules! op_impl {
    ($x: expr, $id: ident) => {{
        let rhs = &*$x.stack.pop().unwrap();
        let lhs = &*$x.stack.pop().unwrap();
        $x.stack.push(rhs.$id(lhs)?)
    }};
}

impl VM {
    pub fn new(inst: Vec<Inst>) -> Self {
        VM {
            head: 0,
            inst,
            stack: Vec::new(),
            heap: Scope::new(),
        }
    }

    pub fn restart(&mut self) {
        self.stack.clear();
        // clear heap?
        self.head = 0
    }

    pub fn end(&self) -> bool {
        self.head >= self.inst.len()
    }

    pub fn run(&mut self) -> Result<Option<Rc<Obj>>, String> {
        while self.head < self.inst.len() {
            if self.execute()? {
                break
            }
        }

        if self.end() {
            let item = self.stack.pop(); // pop and clear the stack
            self.restart();
            return Ok(item)
        }

        Ok(self.stack.pop())
    }

    // Result<false> if the execution should continue
    // Result<true> if the execution should suspend
    fn execute(&mut self) -> Result<bool, String> {
        use vm::inst::Inst::*;

        let inst = &self.inst[self.head];
        self.head += 1;

        match *inst {
            PushInt(i) => self.stack.push(Rc::new(i)),
            PushNum(n) => self.stack.push(Rc::new(n)),
            PushNull => self.stack.push(Rc::new(Null)),
            PushBool(b) => self.stack.push(Rc::new(b)),
            PushStr(ref s) => self.stack.push(Rc::new(s.to_owned())),
            Add => op_impl!(self, add),
            Sub => op_impl!(self, sub),
            Mul => op_impl!(self, mul),
            Div => op_impl!(self, div),
            Get(ref id) => match self.heap.get(id) {
                Some(rc) => self.stack.push(rc),
                None => return Err(String::from("undef var")),
            },
            Store(ref id) => {
                let item = self.stack.pop().unwrap();
                self.heap.insert_rc(id.to_owned(), item);
                // push null?
            }
            Yield => return Ok(true),
            Goto(i) => self.head = i,
            _ => {}
        }

        Ok(false)
    }
}
