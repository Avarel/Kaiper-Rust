use byteorder::{WriteBytesExt, LE};
use vm::inst::Inst;

pub struct InstWriter {
    pub buf: Vec<u8>,
}

macro_rules! stub {
    ($name: ident, $inst: expr $(, $arg: ident: $ty: ty => $method: ident)*) => {
        pub fn $name(&mut self $(, $arg: $ty)*) -> &mut Self {
            self.buf.write_u8($inst as u8).unwrap();
            $(self.buf.$method::<LE>($arg).unwrap();)*
            self
        }
    };
}

impl InstWriter {
    pub fn new() -> Self {
        Self::write_to(Vec::new())
    }

    pub fn write_to(buf: Vec<u8>) -> Self {
        InstWriter { buf }
    }

    pub fn position(&self) -> usize {
        self.buf.len()
    }

    stub!(load_null, Inst::LoadNull);
    stub!(load_int, Inst::LoadInt, int: i32 => write_i32);
    stub!(load_num, Inst::LoadNum, num: f64 => write_f64);
    stub!(load_true, Inst::LoadTrue);
    stub!(load_false, Inst::LoadFalse);
    stub!(load_str, Inst::LoadStr, string_pool_index: u16 => write_u16);
    stub!(store, Inst::Store, table_index: u16 => write_u16, string_pool_index: u16 => write_u16);
    stub!(get, Inst::Get, string_pool_index: u16 => write_u16);
    stub!(ret, Inst::Return);
    stub!(yld, Inst::Yield);

    pub fn invoke(&mut self, arity: u8) -> &mut Self {
        self.buf.write_u8(Inst::Invoke as u8).unwrap();
        self.buf.write_u8(arity).unwrap();
        self
    }

    stub!(push_table, Inst::PushTable);
    stub!(pop_table, Inst::PopTable);
    stub!(jump, Inst::Jump, address: u64 => write_u64);
    stub!(pop_stack, Inst::PopStack);
    stub!(else_jump, Inst::ElseJump, address: u64 => write_u64);

    pub fn complete(self) -> Vec<u8> {
        self.buf
    }
}
