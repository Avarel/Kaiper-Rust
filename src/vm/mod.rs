pub mod inst;
pub mod err;
pub mod compiler;
pub mod frame;

use vm::inst::Inst;
use vm::err::{RTErr, VMErr};
use rt::obj::Obj;
use rt::null::Null;
use std::rc::Rc;
use std::io::Read;
use byteorder::{ReadBytesExt, LE};
use std::io::Cursor;
use vm::frame::VMFrame;

pub struct VM {
    len: u64,
    ptr_offset: u64,
    sp: Vec<String>,
    cursor: Cursor<Vec<u8>>,
}

// pub struct VMFrame {
//     pub ptr: u64,
//     pub end: Option<u64>,
//     pub stack: Vec<Rc<Obj>>,

//     pub heap: VMHeap,
//     pub locals: Vec<Rc<Obj>>,
// }

// impl Default for VMFrame {
//     fn default() -> Self {
//         VMFrame {
//             ptr: 0,
//             end: None,
//             stack: Vec::new(),
//             heap: VMHeap::new(),
//             locals: Vec::new(),
//         }
//     }
// }

type Answer = Result<Option<Rc<Obj>>, VMErr>;
impl VM {
    pub fn load(bytes: Vec<u8>) -> Result<Self, VMErr> {
        let (sp, ptr_offset) = Self::load_string_pool(&bytes)?;

        let program = bytes;
        let len = program.len() as u64;
        let cursor = Cursor::new(program);

        Ok(VM { sp, ptr_offset, len, cursor })
    }

    fn load_string_pool(bytes: &Vec<u8>) -> Result<(Vec<String>, u64), VMErr> {
        let mut cursor = Cursor::new(bytes);
        let string_num = cursor.read_u64::<LE>().map_err(VMErr::IOErr)? as usize;

        let mut string_pool = Vec::with_capacity(string_num);

        for _ in 0..string_num {
            let string_len = cursor.read_u64::<LE>()? as usize;

            let mut buf = vec![0u8; string_len];
            cursor.read_exact(&mut buf)?;

            string_pool.push(String::from_utf8(buf).map_err(|_| VMErr::Internal)?);
        }

        Ok((string_pool, cursor.position()))
    }

    pub fn with_string_pool(inst: Vec<u8>, sp: Vec<String>) -> Self {
        VM {
            len: inst.len() as u64,
            ptr_offset: 0,
            cursor: Cursor::new(inst),
            sp,
        }
    }

    pub fn run(&mut self) -> Answer {
        self.run_context(&mut VMFrame::default())
    }

    pub fn run_context(&mut self, ctx: &mut VMFrame) -> Answer {
        let end = ctx.end.unwrap_or_else(|| self.len);

        self.cursor.set_position(ctx.ptr + self.ptr_offset);

        while self.cursor.position() < end {
            if self.execute(ctx)? {
                break;
            }
        }

        ctx.ptr = self.cursor.position() - self.ptr_offset;

        Ok(ctx.stack.pop())
    }

    fn fetch_inst(&mut self) -> Result<Inst, VMErr> {
        let byte = self.cursor.read_u8()?;
        Inst::from_u8(byte).ok_or_else(|| VMErr::UnknownInstruction(byte))
    }

    fn bin_op(
        &mut self,
        ctx: &mut VMFrame,
        func: fn(&Obj, Rc<Obj>, &mut VM) -> Result<Rc<Obj>, RTErr>,
    ) -> Result<(), VMErr> {
        let rhs = ctx.stack.pop().unwrap();
        let lhs = &*ctx.stack.pop().unwrap();
        ctx.stack.push(func(lhs, rhs, self)?);
        Ok(())
    }

    fn execute(&mut self, ctx: &mut VMFrame) -> Result<bool, VMErr> {
        let inst = self.fetch_inst()?;

        use vm::inst::Inst::*;
        match inst {
            LoadInt => {
                let int = self.cursor.read_i32::<LE>()?;
                ctx.stack.push(Rc::new(int));
            }
            LoadNum => {
                let num = self.cursor.read_f64::<LE>()?;
                ctx.stack.push(Rc::new(num));
            }
            LoadNull => ctx.stack.push(Rc::new(Null)),
            LoadTrue => ctx.stack.push(Rc::new(true)),
            LoadFalse => ctx.stack.push(Rc::new(false)),
            LoadStr => {
                let index = self.cursor.read_u16::<LE>().map_err(VMErr::IOErr)?;
                ctx.stack.push(Rc::new(self.sp[index as usize].to_owned()));
            }
            Add => self.bin_op(ctx, Obj::add)?,
            Sub => self.bin_op(ctx, Obj::sub)?,
            Mul => self.bin_op(ctx, Obj::mul)?,
            Div => self.bin_op(ctx, Obj::div)?,
            GetHeap => {
                let sp_index = self.cursor.read_u16::<LE>()? as usize;
                let id = &self.sp[sp_index];
                match ctx.get_heap(id) {
                    Some(rc) => ctx.stack.push(rc),
                    None => return Err(VMErr::UndefinedVariable(id.to_owned())),
                }
            }
            StoreHeap => {
                let sp_index = self.cursor.read_u16::<LE>()? as usize;
                let obj = ctx.stack.pop().unwrap();
                ctx.set_heap_rc(self.sp[sp_index].to_owned(), obj);
            }
            GetLocal => {
                let local_index = self.cursor.read_u16::<LE>()?;
                let item = ctx.get_local(local_index);
                ctx.stack.push(item);
            }
            StoreLocal => {
                let local_index = self.cursor.read_u16::<LE>()?;
                let item = ctx.stack.pop().unwrap();
                ctx.new_local(local_index, item);
            }
            Delete => {
                let sp_index = self.cursor.read_u16::<LE>()? as usize;
                ctx.delete_heap(&self.sp[sp_index]);
            }
            Invoke => {
                let arity = self.cursor.read_u8()? as usize;

                let mut vec: Vec<Rc<Obj>> = vec![Rc::new(Null); arity];
                for i in 0..arity {
                    let item = ctx.stack.pop().unwrap();
                    vec[arity - i - 1] = item;
                }

                let mut target = &mut ctx.stack.pop().unwrap();
                let result = target.invoke(vec, self).map_err(VMErr::RTErr)?;
                ctx.stack.push(result);
            }
            Yield => return Ok(true),
            JumpIfFalse => {
                let ptr = self.cursor.read_u64::<LE>()?;
                let truth = ctx.stack.pop().unwrap().truth_value();
                if !truth {
                    self.cursor.set_position(ptr + self.ptr_offset);
                }
            }
            Jump => {
                let ptr = self.cursor.read_u64::<LE>()?;
                self.cursor.set_position(ptr + self.ptr_offset);
            }
            PopIgnore => {
                ctx.stack.pop(); // should be dropped
            }
            _ => return Err(VMErr::UnimplementedInstruction),
        }

        Ok(false)
    }
}
