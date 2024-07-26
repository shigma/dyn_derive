use std::{collections::HashMap, fmt::Debug};

use dyn_derive::*;
use dyn_std::Instance;

#[dyn_trait]
pub trait Map<V: Clone>: Clone {
    fn new() -> Self;
    fn get(&self, name: &str) -> Option<&V>;
    fn set(&mut self, name: &str, value: V);
}

#[derive(Debug, Clone)]
pub struct MyMap<V: Clone> {
    store: HashMap<String, V>,
}

impl<V: Clone + 'static> Map<V> for MyMap<V> {
    fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }

    fn get(&self, name: &str) -> Option<&V> {
        self.store.get(name)
    }

    fn set(&mut self, name: &str, value: V) {
        self.store.insert(name.into(), value);
    }
}

#[test]
fn main() {
    let map: &mut dyn MapInstance<i32> = &mut Instance::new(MyMap::new());
    map.set("a", 1);
    map.set("b", 2);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
}
