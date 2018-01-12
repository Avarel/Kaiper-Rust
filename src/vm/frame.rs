use std::rc::Rc;
use std::cell::RefCell;
use rt::obj::Obj;
use std::default::Default;
use std::collections::HashMap;

#[derive(Clone)]
pub struct VMFrame {
    pub ptr: u64,
    pub end: Option<u64>,

    pub stack: Vec<Rc<Obj>>,
    pub locals: Vec<FrameSlot>,
    pub heap: HashMap<String, Rc<Obj>>,
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

    pub fn set_heap<V: Obj>(&mut self, k: String, v: V) -> Option<Rc<Obj>> {
        self.set_heap_rc(k, Rc::new(v))
    }

    pub fn set_heap_rc(&mut self, k: String, v: Rc<Obj>) -> Option<Rc<Obj>> {
        self.heap.insert(k, v)
    }

    pub fn get_heap(&self, k: &String) -> Option<Rc<Obj>> {
        self.heap.get(k).cloned()
    }

    pub fn delete_heap(&mut self, k: &String) -> Option<Rc<Obj>> {
        self.heap.remove(k)
    }

    pub fn set_local(&mut self, index: u16, value: Rc<Obj>) {
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
    pub fn new_local(&mut self, index: u16, value: Rc<Obj>) {
        let index = index as usize;
        if index < self.locals.len() {
            self.locals[index as usize] = FrameSlot::new(value);
        } else if index == self.locals.len() {
            self.locals.push(FrameSlot::new(value));
        } else {
            panic!("VM attempt to set invalid index")
        }
    }

    pub fn get_local(&mut self, index: u16) -> Rc<Obj> {
        self.locals[index as usize].get().clone()
    }
}

#[derive(Clone)]
pub struct FrameSlot {
    value: Rc<RefCell<Rc<Obj>>>,
}

impl FrameSlot {
    fn new(value: Rc<Obj>) -> Self {
        FrameSlot {
            value: Rc::new(RefCell::new(value)),
        }
    }

    fn replace(&self, value: Rc<Obj>) -> Rc<Obj> {
        self.value.replace(value)
    }

    fn get(&self) -> Rc<Obj> {
        RefCell::borrow(&self.value).clone()
    }
}
