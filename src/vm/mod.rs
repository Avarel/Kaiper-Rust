pub mod inst;
pub mod err;

use vm::inst::Inst;
use vm::err::VMErr;
use scope::StackFrames;
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
    pub frames: StackFrames
}

impl Default for VMContext {
    fn default() -> Self {
        VMContext {
            head: 0,
            end: None,
            stack: Vec::new(),
            frames: StackFrames::new(),
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

    pub fn run_context(&mut self, ctx: &mut VMContext) -> Answer {
        let end = ctx.end.unwrap_or_else(|| self.inst.len());
        while ctx.head < end {
            if self.execute(ctx)? {
                break;
            }
        }

        Ok(ctx.stack.pop())
    }

    // Result<false> if the execution should continue
    // Result<true> if the execution should suspend
    fn execute(
        &mut self,
        ctx: &mut VMContext
    ) -> Result<bool, VMErr> {
        macro_rules! op_impl {
            ($stack: expr, $id: ident, $vm: ident) => {{
                let rhs = $stack.pop().unwrap();
                let lhs = &*$stack.pop().unwrap();
                $stack.push(Obj::$id(lhs, rhs, $vm).map_err(|e| VMErr::RtErr(e))?);
            }};
        }

        let inst = self.inst[ctx.head].clone(); // Thinking
        ctx.head += 1;

        use vm::inst::Inst::*;
        match inst {
            LoadInt(i) => ctx.stack.push(Rc::new(i)),
            LoadNum(n) => ctx.stack.push(Rc::new(n)),
            LoadNull => ctx.stack.push(Rc::new(Null)),
            LoadBool(b) => ctx.stack.push(Rc::new(b)),
            LoadStr(ref s) => ctx.stack.push(Rc::new(s.to_owned())),
            Add => op_impl!(ctx.stack, add, self),
            Sub => op_impl!(ctx.stack, sub, self),
            Mul => op_impl!(ctx.stack, mul, self),
            Div => op_impl!(ctx.stack, div, self),
            Get(ref id) => match ctx.frames.get(id) {
                Some(rc) => ctx.stack.push(rc),
                None => return Err(VMErr::UndefinedVariable),
            },
            Store(ref id) => {
                let item = ctx.stack.pop().unwrap();
                ctx.frames.insert_rc(id.to_owned(), item);
            }
            PushTable => ctx.frames.push_frame(),
            PopTable => ctx.frames.pop_frame(),
            Invoke(pop_size) => {
                let mut target = &mut ctx.stack.pop().unwrap();
                let mut vec = Vec::with_capacity(pop_size);
                for _ in 0..pop_size {
                    let what = ctx.stack.pop().unwrap();
                    vec.insert(0, what);
                }
                let result = target.invoke(vec, self).map_err(|e| VMErr::RtErr(e))?;
                ctx.stack.push(result);
            }
            Yield => return Ok(true),
            Jump(i) => ctx.head = i,
            _ => return Err(VMErr::UnknownInstruction),
        }

        Ok(false)
    }
}
