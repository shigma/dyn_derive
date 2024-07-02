use std::collections::HashMap;
use std::fmt::Debug;

use dyn_derive::*;
use dyn_std::Instance;

#[dyn_trait]
pub trait Error: Debug + PartialEq {}

#[dyn_trait]
pub trait Value<#[dynamic] E: Error>: Debug + Clone + PartialEq {
    fn new(v: i32) -> Result<Self, E>;
    fn get(&self) -> i32;
    fn set(&mut self, v: i32) -> Result<(), E>;
}

#[dyn_trait]
pub trait Context<#[dynamic] E: Error, #[dynamic] V: Value<E>> {
    fn get(&self, name: &str) -> Option<V>;
    fn set(&mut self, name: &str, value: V);
}

#[derive(Debug, PartialEq)]
struct MyError;

impl ErrorFactory for MyError {}

#[derive(Debug, Clone, PartialEq)]
struct MyValue(u32);

impl ValueFactory<MyError> for MyValue {
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

pub struct MyContext {
    store: HashMap<String, MyValue>,
}

impl MyContext {
    pub fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

impl ContextFactory<MyError, MyValue> for MyContext {
    fn get(&self, name: &str) -> Option<MyValue> {
        self.store.get(name).cloned()
    }

    fn set(&mut self, name: &str, value: MyValue) {
        self.store.insert(name.to_string(), value);
    }
}

#[test]
fn main() {
    let ctx: &mut dyn Context = &mut Instance::new(MyContext::new());
    let value: &mut dyn Value = &mut Instance::new(MyValue::new(42).unwrap());
    assert_eq!(value.get(), 42);
    assert_eq!(value.set(514), Ok(()));
    assert_eq!(value.get(), 514);
    assert_eq!(value.set(-1), Err(Box::new(Instance::new(MyError)) as Box<dyn Error>));
    assert_eq!(value.get(), 514);
    ctx.set("x", Box::new(Instance::new(MyValue::new(114).unwrap())));
    let value = ctx.get("x").unwrap();
    assert_eq!(value.get(), 114);
}
