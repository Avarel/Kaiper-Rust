pub mod inst;
pub mod err;
pub mod inst_writer;

use vm::inst::Inst;
use vm::err::VMErr;
use scope::VarTables;
use rt::obj::Obj;
use rt::null::Null;
use std::rc::Rc;
use byteorder::{ReadBytesExt, LE};
use std::io::Cursor;

pub struct VM {
    cursor: Cursor<Vec<u8>>,
    len: u64,
    string_pool: Vec<String>,
}

pub struct StackFrame {
    pub head: u64,
    pub end: Option<u64>,
    pub stack: Vec<Rc<Obj>>,
    pub tables: VarTables,
}

impl Default for StackFrame {
    fn default() -> Self {
        StackFrame {
            head: 0,
            end: None,
            stack: Vec::new(),
            tables: VarTables::new(),
        }
    }
}

type Answer = Result<Option<Rc<Obj>>, VMErr>;
impl VM {
    pub fn new(inst: Vec<u8>, string_pool: Vec<String>) -> Self {
        VM {
            len: inst.len() as u64,
            cursor: Cursor::new(inst),
            string_pool,
        }
    }

    pub fn run(&mut self) -> Answer {
        self.run_context(&mut StackFrame::default())
    }

    pub fn run_context(&mut self, ctx: &mut StackFrame) -> Answer {
        let end = ctx.end.unwrap_or_else(|| self.len);

        self.cursor.set_position(ctx.head);

        while self.cursor.position() < end {
            if self.execute(ctx)? {
                break;
            }
        }

        ctx.head = self.cursor.position();

        Ok(ctx.stack.pop())
    }

    // Result<false> if the execution should continue
    // Result<true> if the execution should suspend
    fn execute(&mut self, ctx: &mut StackFrame) -> Result<bool, VMErr> {
        macro_rules! op_impl {
            ($stack: expr, $id: ident, $vm: ident) => {{
                let rhs = $stack.pop().unwrap();
                let lhs = &*$stack.pop().unwrap();
                $stack.push(Obj::$id(lhs, rhs, $vm).map_err(|e| VMErr::RtErr(e))?);
            }};
        }

        let inst = Inst::from_u8(self.cursor.read_u8().map_err(self::map_read_err)?).unwrap(); // Copy semantics
        // ctx.head += 1;

        use vm::inst::Inst::*;
        match inst {
            LoadInt => {
                let int = self.cursor.read_i32::<LE>().map_err(self::map_read_err)?;
                ctx.stack.push(Rc::new(int));
            }
            LoadNum => {
                let num = self.cursor.read_f64::<LE>().map_err(self::map_read_err)?;
                ctx.stack.push(Rc::new(num));
            }
            LoadNull => ctx.stack.push(Rc::new(Null)),
            LoadTrue => ctx.stack.push(Rc::new(true)),
            LoadFalse => ctx.stack.push(Rc::new(false)),
            LoadStr => {
                let index = self.cursor.read_u64::<LE>().map_err(self::map_read_err)?;
                ctx.stack.push(Rc::new(
                    self.string_pool[index as usize].to_owned(),
                ));
            }
            Add => op_impl!(ctx.stack, add, self),
            Sub => op_impl!(ctx.stack, sub, self),
            Mul => op_impl!(ctx.stack, mul, self),
            Div => op_impl!(ctx.stack, div, self),
            Get => {
                let id = self.cursor.read_u64::<LE>().map_err(self::map_read_err)?;
                match ctx.tables.get(&id) {
                    Some(rc) => ctx.stack.push(rc),
                    None => return Err(VMErr::UndefinedVariable),
                }
            }
            Store => {
                let table = self.cursor.read_u64::<LE>().map_err(self::map_read_err)?;
                let table_index = self.cursor.read_u64::<LE>().map_err(self::map_read_err)?;
                let obj = ctx.stack.pop().unwrap();
                ctx.tables.insert_rc_ptr(table as usize, table_index, obj);
            }
            Invoke => {
                let args = self.cursor.read_u64::<LE>().map_err(self::map_read_err)?;
                let mut target = &mut ctx.stack.pop().unwrap();
                let mut vec = Vec::with_capacity(args as usize);
                for _ in 0..args {
                    let what = ctx.stack.pop().unwrap();
                    vec.push(what);
                }
                let result = target.invoke(vec, self).map_err(|e| VMErr::RtErr(e))?;
                ctx.stack.push(result);
            }
            PushTable => ctx.tables.push_table(),
            PopTable => ctx.tables.pop_table(),
            Yield => return Ok(true),
            Jump => {
                let head = self.cursor.read_u64::<LE>().map_err(self::map_read_err)?;
                self.cursor.set_position(head);
            }
            _ => return Err(VMErr::UnknownInstruction),
        }

        Ok(false)
    }
}

use std::io::{Error, ErrorKind};
fn map_read_err(err: Error) -> VMErr {
    match err.kind() {
        ErrorKind::Interrupted => VMErr::InterruptedRead,
        ErrorKind::UnexpectedEof => VMErr::UnexpectedReadEOF,
        _ => VMErr::Internal,
    }
}
