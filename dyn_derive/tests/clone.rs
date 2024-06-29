use std::fmt::Debug;
use dyn_derive::*;

#[dyn_trait]
pub trait Meta: Debug + Clone {
    fn answer(&self) -> i32 {
        42
    }
}

#[derive(Debug, Clone)]
pub struct MetaImpl;

impl MetaConstructor for MetaImpl {}

#[derive(Debug, Clone)]
pub struct Foo {
    meta: Box<dyn Meta>,
}

#[test]
fn main() {
    let foo1 = Foo { meta: Box::new(MetaImpl) };
    let foo2 = foo1.clone();
    drop(foo1);
    assert_eq!(foo2.meta.answer(), 42);
}
