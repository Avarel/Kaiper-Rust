use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Borrow;

pub fn test() {
    let mut scope: Scope<&str, i32> = Scope::new();
    scope.insert("hello", 32);

    scope.insert("what", 32423131);

    [1,2,3][2] = 2;

    borrow(&mut scope);
    clone(&scope);

    println!("should not change {}", scope.get(&"hello").unwrap());

    scope.insert("what", 2);

    println!("{}", scope.get(&"what").unwrap());
}

fn clone(scope: &Scope<&str, i32>) {
    let mut cloned = scope.clone();
    cloned.insert(&"hello", 1000000000);
    println!("from clone {}", cloned.get(&"hello").unwrap());
}

fn borrow<'a, 'b>(scope: &mut Scope<&str, i32>) {
    let mut sub_scope = scope.sub_scope();
    sub_scope.insert("lol", 3);
    sub_scope[&"lol"];
    println!("from parent {}", scope.get(&"hello").unwrap());
}

#[derive(Debug, Clone)]
pub struct Scope<'a, K: 'a + Eq + Hash, V: 'a> {
    parent: Option<&'a Scope<'a, K, V>>,
    map: HashMap<K, V>,
}

impl<'a, K: Eq + Hash, V> Index<&'a K> for Scope<'a, K, V>{
    type Output = V;

    #[inline]
    fn index(&self, index: &K) -> &V {
        self.get(index).expect("no entry found for key")
    }
}

// impl<'a, K: Eq + Hash, V> IndexMut<&'a K> for Scope<'a, K, V> {
//     #[inline]
//     fn index_mut(&mut self, index: &K) -> &mut V {
//         self.get_mut(index).expect("no entry found for key")
//     }
// }

impl<'a, K: Eq + Hash, V> Scope<'a, K, V> {
    fn new() -> Self {
        Scope { parent: None, map: HashMap::new() }
    }

    fn new_with_parent(parent: &'a Scope<'a, K, V>) -> Self {
        Scope { parent: Some(parent), map: HashMap::new() }
    }

    fn sub_scope(&'a self) -> Self {
        Scope::new_with_parent(self)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        let value = self.map.get(key);

        if value.is_none() {
            if let Some(parent) = self.parent {
                let _value = parent.get(key);
                if _value.is_some() {
                    return _value
                }
            }
        }

        value
    }

    // fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    //     let value = self.map.get_mut(key);

    //     if value.is_none() {
    //         if let Some(parent) = self.parent.as_mut() {
    //             let _value = parent.get_mut(key);
    //             if _value.is_some() {
    //                 return _value
    //             }
    //         }
    //     }

    //     value
    // }
}