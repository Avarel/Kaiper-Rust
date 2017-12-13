pub mod inst;

use vm::inst::Inst;
use scope::Scope;
use std::borrow::{Borrow, BorrowMut};
use rt::obj::Obj;
use rt::null::Null;
use std::rc::Rc;
use std::cell::RefCell;

pub struct VM {
    index: usize, // instruction index
    inst: Vec<Inst>,
    stack: Rc<RefCell<Vec<Rc<Box<Obj>>>>>,
    heap: Rc<Scope<String, Box<Obj>>>,
}

impl VM {
    pub fn new(inst: Vec<Inst>) -> Self {
        VM {
            index: 0,
            inst,
            stack: Rc::new(RefCell::new(Vec::new())),
            heap: Rc::new(Scope::new()),
        }
    }

    pub fn run(&mut self) -> Result<Rc<Box<Obj>>, String> {
        while self.index < self.inst.len() {
            self.execute()?;
        }

        Ok(RefCell::borrow_mut(&self.stack).pop().unwrap_or_else(|| Rc::new(Box::new(Null))))
    }

    fn execute(&mut self) -> Result<(), String> {
        use vm::inst::Inst::*;
        match self.inst[self.index] {
            PushInt(i) => RefCell::borrow_mut(&self.stack).push(Rc::new(Box::new(i))),
            PushNum(n) => RefCell::borrow_mut(&self.stack).push(Rc::new(Box::new(n))),
            PushNull => RefCell::borrow_mut(&self.stack).push(Rc::new(Box::new(Null))),
            Add => {
                let rhs = &*RefCell::borrow_mut(&self.stack).pop().unwrap();
                let lhs = &*RefCell::borrow_mut(&self.stack).pop().unwrap();
                RefCell::borrow_mut(&self.stack).push(Rc::new(rhs.add(lhs.borrow())?))
            }
            Get(ref id) => {
                let id = id.clone();
            }
            Goto(i) => {
                self.index = i;
                return Ok(());
            }
            _ => {}
        }
        self.index += 1;

        Ok(())
    }
}

/*
pub struct VM {
    index: usize, // instruction index
    inst: Vec<Inst>,
}

impl VM {
    pub fn new(inst: Vec<Inst>) -> Self {
        VM {
            index: 0,
            inst,
        }
    }

    pub fn run(&mut self) -> Result<Rc<Box<Obj>>, String> {
        let mut stack = Vec::new();
        let heap = Scope::new();

        while self.index < self.inst.len() {
            self.execute(&mut stack, &heap)?;
        }

        Ok(stack.pop().unwrap_or_else(|| Rc::new(Box::new(Null))))
    }

    fn execute(
        &mut self,
        stack: &mut Vec<Rc<Box<Obj>>>,
        heap: &Scope<String, Box<Obj>>,
    ) -> Result<(), String> {
        use vm::inst::Inst::*;
        match self.inst[self.index] {
            PushInt(i) => stack.push(Rc::new(Box::new(i))),
            PushNum(n) => stack.push(Rc::new(Box::new(n))),
            PushNull => stack.push(Rc::new(Box::new(Null))),
            Add => {
                let rhs = &*stack.pop().unwrap();
                let lhs = &*stack.pop().unwrap();
                stack.push(Rc::new(rhs.add(lhs.borrow())?))
            }
            Get(ref id) => {
                let id = id.to_owned();
            }
            Goto(i) => {
                self.index = i;
                return Ok(());
            }
            _ => {}
        }
        self.index += 1;

        Ok(())
    }
}
*/