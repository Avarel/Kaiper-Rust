// eventually migrate to binary
#[derive(Clone, Copy)]
pub enum Inst {
    // Load a null reference onto the stack
    LoadNull = 0,

    // Load an int onto the stack
    LoadInt = 1, // i32
    // Load a number onto the stack
    LoadNum = 2, // f64

    // Load a boolean onto the stack
    LoadTrue = 3, // u8
    LoadFalse = 4,

    // Load a string onto the stack
    LoadStr = 5, // u64

    // Pop the stack and store an obj onto the heap
    Store = 6, // u64, u64
    // Get and push an obj on the heap onto the stack
    Get = 7, // u64

    // Pop and return an answer, stopping
    Return = 8,
    // Pop and yield an answer, suspending the head
    Yield = 9,

    // Pop 2 from the stack and do the operation
    Add = 10,
    Sub = 11,
    Mul = 12,
    Div = 13,

    Invoke = 14, // u64

    // Goto location
    Jump = 15, // u64

    PushTable = 16,
    PopTable = 17,

    // Just pop a value off a stack and ignore it
    PopStack = 18,
}

impl Inst {
    pub fn from_u8(byte: u8) -> Option<Inst> {
        use vm::inst::Inst::*;
        Some(match byte {
            0 => LoadNull,
            1 => LoadInt,
            2 => LoadNum,
            3 => LoadTrue,
            4 => LoadFalse,
            5 => LoadStr,
            6 => Store,
            7 => Get,
            8 => Return,
            9 => Yield,
            10 => Add,
            11 => Sub,
            12 => Mul,
            13 => Div,
            14 => Invoke,
            15 => Jump,
            16 => PushTable,
            17 => PopTable,
            18 => PopStack,
            _ => return None,
        })
    }
}
