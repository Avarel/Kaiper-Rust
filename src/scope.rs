use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use std::collections::HashMap;
use std::hash::Hash;

/// Scopes are wrapper structs for a map that can have fallbacks.
/// In this implementation, the last map is always the scope's
/// immediate map.
pub struct Scope<K, V> {
    pub maps: Vec<Rc<RefCell<HashMap<K, Rc<V>>>>>,
}

impl<K: Eq + Hash, V> Scope<K, V> {
    /// Creates a new scope.
    pub fn new() -> Self {
        Scope {
            maps: vec![Rc::new(RefCell::new(HashMap::new()))],
        }
    }

    pub fn copy(&self) -> Self {
        Scope { maps: self.maps.clone() }
    }

    /// Creates a new scope with this scope as fallback.
    /// The new scope also inherits the current scope's fallbacks.
    pub fn sub_scope(&self) -> Self {
        let mut maps = self.maps.clone();
        maps.push(Rc::new(RefCell::new(HashMap::new())));
        Scope { maps }
    }

    /// Insert a value into the immediate scope.
    pub fn insert(&self, k: K, v: V) -> Option<Rc<V>> {
        self.hash_map_mut().insert(k, Rc::new(v))
    }

    /// Get a value from the scope.
    /// If the value doesn't exist in the scope, use fallback
    /// values from fallback scopes (which may or may not exist).
    pub fn get(&self, k: &K) -> Option<Rc<V>> {
        self.maps
            .iter()
            .rev()
            .map(|rc| RefCell::borrow(rc))
            .map(|map| map.get(k).map(|opt| opt.clone()))
            .find(|opt| opt.is_some())?
    }

    /// Returns if the scope and the parent scopes contain the key.
    pub fn any_contains(&self, k: &K) -> bool {
        self.maps
            .iter()
            .rev()
            .map(|rc| RefCell::borrow(rc))
            .any(|map| map.contains_key(k))
    }

    /// Returns if the immediate scope contains the key.
    pub fn map_contains(&self, k: &K) -> bool {
        self.hash_map().contains_key(k)
    }

    /// Returns a reference to the immediate HashMap.
    pub fn hash_map(&self) -> Ref<HashMap<K, Rc<V>>> {
        RefCell::borrow(self.maps.last().unwrap())
    }

    // FOOL, you've never seen hacks like THESE
    /// Returns a mutable reference to the immediate HashMap.
    pub fn hash_map_mut(&self) -> RefMut<HashMap<K, Rc<V>>> {
        RefCell::borrow_mut(self.maps.last().unwrap())
    }
}
