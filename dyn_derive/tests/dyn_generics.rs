use std::collections::HashMap;
use std::fmt::Debug;

use dyn_derive::*;
use dyn_std::Instance;

#[dyn_trait]
pub trait Error: Debug + PartialEq {}

#[dyn_trait]
pub trait Value: Debug + Clone + PartialEq {
    #[dyn_trait]
    type E: Error;
    fn new(v: i32) -> Result<Self, Self::E>;
    fn get(&self) -> i32;
    fn set(&mut self, v: i32) -> Result<(), Self::E>;
}

#[dyn_trait]
pub trait Context {
    #[dyn_trait]
    type V: Value;
    fn get(&self, name: &str) -> Option<Self::V>;
    fn set(&mut self, name: &str, value: Self::V);
    fn extend(&mut self, store: HashMap<String, Self::V>);
}

#[derive(Debug, PartialEq)]
struct MyError;

impl Error for MyError {}

#[derive(Debug, Clone, PartialEq)]
struct MyValue(u32);

impl Value for MyValue {
    type E = MyError;

    fn new(v: i32) -> Result<Self, MyError> {
        if v < 0 {
            Err(MyError)
        } else {
            Ok(Self(v as u32))
        }
    }

    fn get(&self) -> i32 {
        self.0 as i32
    }

    fn set(&mut self, v: i32) -> Result<(), MyError> {
        if v < 0 {
            Err(MyError)
        } else {
            self.0 = v as u32;
            Ok(())
        }
    }
}

struct MyContext {
    store: HashMap<String, MyValue>,
}

impl MyContext {
    pub fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

impl Context for MyContext {
    type V = MyValue;

    fn get(&self, name: &str) -> Option<MyValue> {
        self.store.get(name).cloned()
    }

    fn set(&mut self, name: &str, value: MyValue) {
        self.store.insert(name.to_string(), value);
    }

    fn extend(&mut self, store: HashMap<String, MyValue>) {
        self.store.extend(store)
    }
}

#[test]
fn main() {
    let ctx: &mut dyn ContextInstance = &mut Instance::new(MyContext::new());
    let value: &mut dyn ValueInstance = &mut Instance::new(MyValue::new(42).unwrap());
    assert_eq!(value.get(), 42);
    assert_eq!(value.set(514), Ok(()));
    assert_eq!(value.get(), 514);
    assert_eq!(value.set(-1), Err(Box::new(Instance::new(MyError)) as Box<dyn ErrorInstance>));
    assert_eq!(value.get(), 514);
    ctx.set("x", Box::new(Instance::new(MyValue::new(114).unwrap())));
    let value = ctx.get("x").unwrap();
    assert_eq!(value.get(), 114);
}
