pub mod inst;
pub mod err;

use vm::inst::Inst;
use vm::err::VMErr;
use scope::Scope;
use rt::obj::Obj;
use rt::null::Null;
use std::rc::Rc;

pub struct VM {
    inst: Vec<Inst>,
}

pub struct VMContext {
    head: usize,
    end: Option<usize>,
    stack: Vec<Rc<Obj>>,
    heap: Scope,
}

type Answer = Result<Option<Rc<Obj>>, VMErr>;

impl Default for VMContext {
    fn default() -> Self {
        VMContext {
            head: 0,
            end: None,
            stack: Vec::new(),
            heap: Scope::new(),
        }
    }
}

impl VM {
    pub fn new(inst: Vec<Inst>) -> Self {
        VM { inst }
    }

    pub fn run(&mut self) -> Answer {
        self.run_context(&mut VMContext::default())
    }

    // Should be run by normal things
    pub fn run_context(&mut self, c: &mut VMContext) -> Answer {
        self.run_impl(c, false)
    }

    // Should be run by generator functions or coroutines
    pub fn run_continuation(&mut self, c: &mut VMContext) -> Answer {
        self.run_impl(c, true)
    }

    fn run_impl(&mut self, c: &mut VMContext, continuation: bool) -> Answer {
        let end = c.end.unwrap_or_else(|| self.inst.len());
        while c.head < end {
            if self.execute(&mut c.head, &mut c.stack, &mut c.heap, continuation)? {
                break;
            }
        }

        Ok(c.stack.pop())
    }

    // Result<false> if the execution should continue
    // Result<true> if the execution should suspend
    fn execute(
        &mut self,
        head: &mut usize,
        stack: &mut Vec<Rc<Obj>>,
        heap: &mut Scope,
        continuation: bool,
    ) -> Result<bool, VMErr> {
        use vm::inst::Inst::*;

        macro_rules! op_impl {
            ($stack: ident, $id: ident) => {{
                let rhs = &*$stack.pop().unwrap();
                let lhs = &*$stack.pop().unwrap();
                $stack.push(rhs.$id(lhs).map_err(|e| VMErr::RtErr(e))?)
            }};
        }

        let inst = &self.inst[*head];
        *head += 1;

        match *inst {
            PushInt(i) => stack.push(Rc::new(i)),
            PushNum(n) => stack.push(Rc::new(n)),
            PushNull => stack.push(Rc::new(Null)),
            PushBool(b) => stack.push(Rc::new(b)),
            PushStr(ref s) => stack.push(Rc::new(s.to_owned())),
            Add => op_impl!(stack, add),
            Sub => op_impl!(stack, sub),
            Mul => op_impl!(stack, mul),
            Div => op_impl!(stack, div),
            Get(ref id) => match heap.get(id) {
                Some(rc) => stack.push(rc),
                None => return Err(VMErr::UndefinedVariable),
            },
            Store(ref id) => {
                let item = stack.pop().unwrap();
                heap.insert_rc(id.to_owned(), item);
                // push null?
            }
            Yield if continuation => return Ok(true),
            Jump(i) => *head = i,
            _ => return Err(VMErr::UnknownInstruction),
        }

        Ok(false)
    }
}
