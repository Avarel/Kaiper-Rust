use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;
use std::hash::Hash;

pub struct Scope<K, V> {
    pub maps: Vec<Rc<RefCell<HashMap<K, Rc<V>>>>>,
}

impl<K: Eq + Hash, V> Scope<K, V> {
    pub fn new() -> Self {
        Scope {
            maps: vec![Rc::new(RefCell::new(HashMap::new()))],
        }
    }

    pub fn sub_scope(&self) -> Self {
        let mut maps = self.maps.clone();
        maps.push(Rc::new(RefCell::new(HashMap::new())));
        Scope { maps }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<Rc<V>> {
        RefCell::borrow_mut(self.maps.last().unwrap()).insert(k, Rc::new(v))
    }

    pub fn get(&self, k: &K) -> Option<Rc<V>> {
        self.maps
            .iter()
            .rev()
            .map(|rc| RefCell::borrow(rc))
            .map(|map| map.get(k).map(|opt| opt.clone()))
            .find(|opt| opt.is_some())?
    }
}
