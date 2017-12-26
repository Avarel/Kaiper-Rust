use byteorder::{WriteBytesExt, LE};
use vm::inst::Inst;

pub struct InstWriter {
    wtr: Vec<u8>,
}

macro_rules! stub {
    ($name: ident, $inst: expr $(, $arg: ident: $ty: ty => $method: ident)*) => {
        pub fn $name(&mut self $(, $arg: $ty)*) -> &mut Self {
            self.wtr.write_u8($inst as u8).unwrap();
            $(self.wtr.$method::<LE>($arg).unwrap();)*
            self
        }
    };
}

impl InstWriter {
    pub fn new() -> Self {
        Self::write_to(Vec::new())
    }

    pub fn write_to(wtr: Vec<u8>) -> Self {
        InstWriter { wtr }
    }

    pub fn position(&self) -> usize {
        self.wtr.len()
    }

    stub!(load_null, Inst::LoadNull);
    stub!(load_int, Inst::LoadInt, int: i32 => write_i32);
    stub!(load_num, Inst::LoadNum, num: f64 => write_f64);
    stub!(load_true, Inst::LoadTrue);
    stub!(load_false, Inst::LoadFalse);
    stub!(load_str, Inst::LoadStr, string_pool_index: u64 => write_u64);
    stub!(store, Inst::Store, table: u64 => write_u64, string_pool_index: u64 => write_u64);
    stub!(get, Inst::Get, string_pool_index: u64 => write_u64);
    stub!(ret, Inst::Return);
    stub!(yld, Inst::Yield);
    stub!(invoke, Inst::Invoke, args: u64 => write_u64);
    stub!(push_table, Inst::PushTable);
    stub!(pop_table, Inst::PopTable);
    stub!(jump, Inst::Jump, address: u64 => write_u64);
    stub!(pop_stack, Inst::PopStack);

    pub fn complete(self) -> Vec<u8> {
        self.wtr
    }
}
