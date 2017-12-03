use std::cell::RefCell;
use std::rc::Rc;

pub fn test() {
    let mut scope1: Scope<&str, &i32> = Scope::new();

    scope1.insert("wow", &213);

    let mut please = scope1.sub_scope();

    {
        please.insert("what", &23);

        declare(&mut scope1);

        // let borrow: &i32 = please.get(&"what").unwrap();
        // let borrow = please.get(&"what").unwrap();
        println!("plase[wow] {}", please.get(&"wow").unwrap());

        println!("scope[what] {}", scope1.get(&"wow").unwrap());

        please.insert("what", &23);

        println!("please[what] = {}", please.get(&"what").unwrap());

        println!("scope[what] = {}", scope1.get(&"what").unwrap());
    }
}

fn declare(scope: &mut Scope<&str, &i32>) {
    let mut sub_scope = scope.sub_scope();
    sub_scope.insert("what", &234);

    scope.insert("what", &10000);
}

use std::collections::HashMap;
use std::hash::Hash;

pub struct Scope<K, V> {
    pub maps: Vec<Rc<RefCell<HashMap<K, Rc<V>>>>>,
}

impl<K: Eq + Hash, V> Scope<K, V> {
    pub fn new() -> Self {
        Scope { maps: vec![Rc::new(RefCell::new(HashMap::new()))] }
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
