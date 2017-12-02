use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Borrow;
use std::mem;

pub fn test() {
    let mut scope1: Scope<&str, &i32> = Scope::new();

    scope1.insert("wow", &213);

    let mut please = scope1.sub_scope();

    {
        please.insert("what", &23);

        declare(&mut scope1);

        let borrow = please.get(&"what").unwrap();
        println!("{}", borrow.borrow_mut());
        
        let borrow = scope1.get(&"what").unwrap();
        println!("{}", borrow.borrow_mut());

        let borrow = please.get(&"wow").unwrap();
        println!("{}", borrow.borrow_mut());
    }
}

fn declare(scope: &mut Scope<&str, &i32>) {
    let mut sub_scope = scope.sub_scope();
    sub_scope.insert("what", &234);

    scope.insert("what", &324);
}

#[derive(Clone)]
pub struct Scope<K, V> {
    // Do you believe in a god?
    maps: Vec<Rc<RefCell<Vec<(K, Rc<RefCell<V>>)>>>>,
}

// This is what insanity looks like.
impl<K: Eq, V> Scope<K, V> {
    pub fn new() -> Self {
        Scope {
            maps: vec![Rc::new(RefCell::new(vec![]))],
        }
    }

    pub fn sub_scope(&self) -> Self {
        let mut maps = self.maps.clone();
        maps.push(Rc::new(RefCell::new(vec![])));
        Scope { maps: maps }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<Rc<RefCell<V>>> {
        let last_index = self.maps.len() - 1;
        let mut last_vec = self.maps[last_index].borrow_mut();
        if let Some(index) = last_vec.iter().position(|tuple| tuple.0 == k) {
            let tuple = mem::replace(&mut last_vec[index], (k, Rc::new(RefCell::new(v))));
            return Some(tuple.1.clone());
        } else {
            last_vec.push((k, Rc::new(RefCell::new(v))));
            return None;
        }
    }

    pub fn get(&self, k: &K) -> Option<Rc<RefCell<V>>> {
        for i in (0..self.maps.len()).rev() {
            let vec = RefCell::borrow(self.maps[i].borrow());
            if let Some(val) = vec.iter().find(|tuple| (*tuple).0 == *k) {
                return Some(val.1.clone());
            }
        }
        None
    }
}
