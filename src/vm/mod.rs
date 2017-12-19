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
    pub head: usize,
    pub end: Option<usize>,
    pub stack: Vec<Rc<Obj>>,
    pub heap: Scope
}

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

type Answer = Result<Option<Rc<Obj>>, VMErr>;
impl VM {
    pub fn new(inst: Vec<Inst>) -> Self {
        VM { inst: inst }
    }

    pub fn run(&mut self) -> Answer {
        self.run_context(&mut VMContext::default())
    }

    pub fn run_context(&mut self, c: &mut VMContext) -> Answer {
        let end = c.end.unwrap_or_else(|| self.inst.len());
        while c.head < end {
            if self.execute(&mut c.head, &mut c.stack, &mut c.heap)? {
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
    ) -> Result<bool, VMErr> {
        macro_rules! op_impl {
            ($stack: ident, $id: ident, $vm: ident) => {{
                let rhs = $stack.pop().unwrap();
                let lhs = &*$stack.pop().unwrap();
                $stack.push(Obj::$id(lhs, rhs, $vm).map_err(|e| VMErr::RtErr(e))?);
            }};
        }

        let inst = self.inst[*head].clone(); // Thinking
        *head += 1;

        use vm::inst::Inst::*;
        match inst {
            PushInt(i) => stack.push(Rc::new(i)),
            PushNum(n) => stack.push(Rc::new(n)),
            PushNull => stack.push(Rc::new(Null)),
            PushBool(b) => stack.push(Rc::new(b)),
            PushStr(ref s) => stack.push(Rc::new(s.to_owned())),
            Add => op_impl!(stack, add, self),
            Sub => op_impl!(stack, sub, self),
            Mul => op_impl!(stack, mul, self),
            Div => op_impl!(stack, div, self),
            Get(ref id) => match heap.get(id) {
                Some(rc) => stack.push(rc),
                None => return Err(VMErr::UndefinedVariable),
            },
            Store(ref id) => {
                let item = stack.pop().unwrap();
                heap.insert_rc(id.to_owned(), item);
            }
            Invoke(pop_size) => {
                let mut target = &mut stack.pop().unwrap();
                let mut vec = Vec::with_capacity(pop_size);
                for _ in 0..pop_size {
                    let what = stack.pop().unwrap();
                    vec.insert(0, what);
                }
                let result = target.invoke(vec, self).map_err(|e| VMErr::RtErr(e))?;
                stack.push(result);
            }
            Yield => return Ok(true),
            Jump(i) => *head = i,
            _ => return Err(VMErr::UnknownInstruction),
        }

        Ok(false)
    }
}
