// #[derive(Clone,)]
pub enum Inst {
    // Push an int onto the stack
    PushInt(i32),
    // Push a number onto the stack
    PushNum(f64),
    // Push a null reference onto the stack
    PushNull,
    // Push a string onto the stack
    PushStr(String),
    // Push a boolean onto the stack
    PushBool(bool),

    // Pop the stack and store an obj onto the heap
    Store(String),
    // Get and push an obj on the heap onto the stack
    Get(String),

    // Pop 2 from the stack and do the operation
    Add,
    Sub,
    Mul,
    Div,

    // Goto location
    Goto(usize),
}