use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use std::collections::HashMap;
use rt::obj::Obj;

pub struct StackFrames {
    pub frames: Vec<Rc<RefCell<HashMap<String, Rc<Obj>>>>>,
}

impl StackFrames {
    pub fn new() -> Self {
        StackFrames { 
            frames: vec![Rc::new(RefCell::new(HashMap::new()))] 
        }
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn push_frame(&mut self) {
        self.frames.push(Rc::new(RefCell::new(HashMap::new())));
    }

    /// Insert a value into the immediate scope.
    pub fn insert<T: Obj>(&mut self, k: String, v: T) -> Option<Rc<Obj>> {
        self.insert_rc(k, Rc::new(v))
    }

    pub fn insert_rc(&mut self, k: String, v: Rc<Obj>) -> Option<Rc<Obj>> {
        self.hash_map_mut().insert(k, v)
    }

    /// Get a value from the scope.
    /// If the value doesn't exist in the scope, use fallback
    /// values from fallback scopes (which may or may not exist).
    pub fn get(&self, k: &String) -> Option<Rc<Obj>> {
        self.frames
            .iter()
            .rev()
            .map(|rc| RefCell::borrow(rc))
            .map(|map| map.get(k).map(|opt| opt.clone()))
            .find(|opt| opt.is_some())?
    }

    /// Returns if the scope and the parent scopes contain the key.
    pub fn any_contains(&self, k: &String) -> bool {
        self.frames
            .iter()
            .rev()
            .map(|rc| RefCell::borrow(rc))
            .any(|map| map.contains_key(k))
    }

    /// Returns if the immediate scope contains the key.
    pub fn map_contains(&self, k: &String) -> bool {
        self.hash_map().contains_key(k)
    }

    /// Returns a reference to the immediate HashMap.
    pub fn hash_map(&self) -> Ref<HashMap<String, Rc<Obj>>> {
        RefCell::borrow(self.frames.last().unwrap())
    }

    // FOOL, you've never seen hacks like THESE
    /// Returns a mutable reference to the immediate HashMap.
    pub fn hash_map_mut(&self) -> RefMut<HashMap<String, Rc<Obj>>> {
        RefCell::borrow_mut(self.frames.last().unwrap())
    }
}

// /// Scopes are wrapper structs for a map that can have fallbacks.
// /// In this implementation, the last map is always the scope's
// /// immediate map.
// pub struct Scope {
//     pub maps: Vec<Rc<RefCell<HashMap<String, Rc<Obj>>>>>,
// }

// impl Scope {
//     /// Creates a new scope.
//     pub fn new() -> Self {
//         Scope {
//             maps: vec![Rc::new(RefCell::new(HashMap::new()))],
//         }
//     }

//     /// Creates a new scope with this scope as fallback.
//     /// The new scope also inherits the current scope's fallbacks.
//     pub fn sub_scope(&self) -> Self {
//         let mut maps = self.maps.clone();
//         maps.push(Rc::new(RefCell::new(HashMap::new())));
//         Scope { maps }
//     }

//     /// Insert a value into the immediate scope.
//     pub fn insert<T: Obj>(&self, k: String, v: T) -> Option<Rc<Obj>> {
//         self.insert_rc(k, Rc::new(v))
//     }

//     pub fn insert_rc(&self, k: String, v: Rc<Obj>) -> Option<Rc<Obj>> {
//         self.hash_map_mut().insert(k, v)
//     }

//     /// Get a value from the scope.
//     /// If the value doesn't exist in the scope, use fallback
//     /// values from fallback scopes (which may or may not exist).
//     pub fn get(&self, k: &String) -> Option<Rc<Obj>> {
//         self.maps
//             .iter()
//             .rev()
//             .map(|rc| RefCell::borrow(rc))
//             .map(|map| map.get(k).map(|opt| opt.clone()))
//             .find(|opt| opt.is_some())?
//     }

//     /// Returns if the scope and the parent scopes contain the key.
//     pub fn any_contains(&self, k: &String) -> bool {
//         self.maps
//             .iter()
//             .rev()
//             .map(|rc| RefCell::borrow(rc))
//             .any(|map| map.contains_key(k))
//     }

//     /// Returns if the immediate scope contains the key.
//     pub fn map_contains(&self, k: &String) -> bool {
//         self.hash_map().contains_key(k)
//     }

//     /// Returns a reference to the immediate HashMap.
//     pub fn hash_map(&self) -> Ref<HashMap<String, Rc<Obj>>> {
//         RefCell::borrow(self.maps.last().unwrap())
//     }

//     // FOOL, you've never seen hacks like THESE
//     /// Returns a mutable reference to the immediate HashMap.
//     pub fn hash_map_mut(&self) -> RefMut<HashMap<String, Rc<Obj>>> {
//         RefCell::borrow_mut(self.maps.last().unwrap())
//     }
// }

impl Clone for StackFrames {
    fn clone(&self) -> Self {
        StackFrames { frames: self.frames.clone() }
    }
}