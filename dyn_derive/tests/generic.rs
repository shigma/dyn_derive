use std::fmt::Debug;

use dyn_derive::*;
use dyn_std::Instance;

#[dyn_trait]
pub trait Error: Debug + PartialEq {}

#[dyn_trait]
pub trait Value<E: Error>: Debug + PartialEq {
    fn new(v: i32) -> Result<Self, E>;
    fn get(&self) -> i32;
    fn set(&mut self, v: i32) -> Result<(), E>;
}

#[derive(Debug, PartialEq)]
struct MyError;

impl ErrorStatic for MyError {}

#[derive(Debug, PartialEq)]
struct MyValue(u32);

impl ValueStatic<MyError> for MyValue {
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

#[test]
fn main() {
    let v: &mut dyn Value = &mut Instance::new(MyValue::new(42).unwrap());
    assert_eq!(v.get(), 42);
    assert_eq!(v.set(514), Ok(()));
    assert_eq!(v.get(), 514);
    assert_eq!(v.set(-1), Err(Box::new(Instance::new(MyError)) as Box<dyn Error>));
    assert_eq!(v.get(), 514);
}
