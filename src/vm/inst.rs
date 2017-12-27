// eventually migrate to binary
#[derive(Debug, Clone, Copy)]
pub enum Inst {
    // Documentation
    // (byte count) 
    // (following bytes...)

    // Load a null reference onto the stack.
    // 1 byte.
    LoadNull = 0,

    // Load an int onto the stack.
    // 5 bytes.
    // int: i32
    LoadInt = 1,

    // Load a number onto the stack.
    // 9 bytes. 
    // number: f64
    LoadNum = 2, 

    // Load a boolean onto the stack.
    // 1 byte.
    LoadTrue = 3,
    LoadFalse = 4,

    // Load a string with the index the string pool onto the stack.
    // 3 bytes.
    // string_pool_index: u16
    LoadStr = 5,

    // Pop the stack and store an object onto the variable tables.
    // 5 bytes.
    // table_index: u16, string_pool_index: u16
    Store = 6, 

    // Get and push an object on the heap onto the stack.
    // 3 bytes.
    // string_pool_index: u16
    Get = 7,

    // Pop and return an answer, stopping execution.
    // 1 byte.
    Return = 8,
    // Pop and yield an answer, suspending the instruction pointer.
    // 1 byte.
    Yield = 9,

    // Pop 2 objects from the stack and perform the operation.
    // 1 byte.
    Add = 10,
    Sub = 11,
    Mul = 12,
    Div = 13,

    // Pop (arity + 1) number of objects from the stack and perform invocation operation.
    // 2 byte.
    // arity: u8
    Invoke = 14,

    // Set the instruction pointer.
    // 9 bytes.
    // address: u64
    Jump = 15,

    // Pop the stack and set the instruction pointer if the truth value of the pop is false.
    // 9 bytes.
    // address: u64
    ElseJump = 19, 

    // Push a variable table.
    // 1 byte.
    PushTable = 16,

    // Pop a variable table.
    // 1 byte.
    PopTable = 17,

    // Pop a value off a stack and ignore it.
    // 1 byte.
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
            19 => ElseJump,
            _ => return None,
        })
    }
}
