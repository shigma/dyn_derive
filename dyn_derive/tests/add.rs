use std::fmt::Debug;
use std::ops::Add;
use dyn_derive::*;

#[dyn_trait]
pub trait Meta: Debug + Add {}

#[derive(Debug)]
pub struct MetaImpl(String);

impl Meta for MetaImpl {}

impl Add for MetaImpl {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + &rhs.0)
    }
}

pub struct Foo {
    meta: Box<dyn Meta>,
}

impl Add for Foo {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { meta: self.meta + rhs.meta }
    }
}

#[test]
fn main() {
    let foo1 = Foo { meta: Box::new(MetaImpl("114".into())) };
    let foo2 = Foo { meta: Box::new(MetaImpl("514".into())) };
    let foo3 = foo1 + foo2;
    assert_eq!(format!("{:?}", foo3.meta), "MetaImpl(\"114514\")");
}
