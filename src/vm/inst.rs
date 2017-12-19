#[derive(Clone)]
pub enum Inst {
    // Load an int onto the stack
    LoadInt(i32),
    // Load a number onto the stack
    LoadNum(f64),
    // Load a null reference onto the stack
    LoadNull,
    // Load a string onto the stack
    LoadStr(String),
    // Load a boolean onto the stack
    LoadBool(bool),

    // Pop the stack and store an obj onto the heap
    Store(String),
    // Get and push an obj on the heap onto the stack
    Get(String),

    // Pop and return an answer, stopping
    Return,
    // Pop and yield an answer, suspending the head
    Yield,

    // Pop 2 from the stack and do the operation
    Add,
    Sub,
    Mul,
    Div,
    
    Invoke(usize),

    // Goto location
    Jump(usize),

    PushTable,
    PopTable,
}