use std::rc::Rc;
use std::cell::RefCell;
use rt::Obj;
use std::default::Default;
use std::collections::HashMap;

// Frame factory?

#[derive(Clone)]
pub struct VMFrame {
    pub ptr: u64,
    pub end: Option<u64>,

    pub stack: Vec<Obj>,
    pub locals: Vec<FrameSlot>,
    pub heap: HashMap<Rc<String>, Obj>,
}

impl Default for VMFrame {
    fn default() -> Self {
        VMFrame {
            ptr: 0,
            end: None,
            stack: Vec::new(),
            locals: Vec::new(),
            heap: HashMap::new(),
        }
    }
}

impl VMFrame {
    fn new() -> Self {
        VMFrame::default()
    }

    pub fn set_heap(&mut self, k: Rc<String>, v: Obj) {
        self.heap.insert(k, v);
    }

    pub fn get_heap(&self, k: &String) -> Option<Obj> {
        self.heap.get(k).cloned()
    }

    pub fn delete_heap(&mut self, k: &String) {
        self.heap.remove(k);
    }

    pub fn set_local(&mut self, index: u16, value: Obj) {
        let index = index as usize;
        if index < self.locals.len() {
            self.locals[index as usize].replace(value);
        } else if index == self.locals.len() {
            self.locals.push(FrameSlot::new(value));
        } else {
            panic!("VM attempt to set invalid index")
        }
    }

    // abandons the old slot (which might be kept by derived frames) and sets a new slot with the value
    pub fn new_local(&mut self, index: u16, value: Obj) {
        let index = index as usize;
        if index < self.locals.len() {
            self.locals[index as usize] = FrameSlot::new(value);
        } else if index == self.locals.len() {
            self.locals.push(FrameSlot::new(value));
        } else {
            panic!("VM attempt to set invalid index")
        }
    }

    pub fn get_local(&mut self, index: u16) -> Obj {
        self.locals[index as usize].get().clone()
    }
}

#[derive(Clone)]
pub struct FrameSlot {
    value: Rc<RefCell<Obj>>,
}

impl FrameSlot {
    fn new(value: Obj) -> Self {
        FrameSlot {
            value: Rc::new(RefCell::new(value)),
        }
    }

    fn replace(&self, value: Obj) -> Obj {
        self.value.replace(value)
    }

    fn get(&self) -> Obj {
        RefCell::borrow(&self.value).clone()
    }
}
