use std::fmt::Debug;

use dyn_derive::*;

#[dyn_trait]
pub trait Meta: Debug + PartialEq {}

#[derive(Debug, PartialEq)]
pub struct MetaImpl1(i32);

impl MetaInstance for MetaImpl1 {}

#[derive(Debug, PartialEq)]
pub struct MetaImpl2(i32);

impl MetaInstance for MetaImpl2 {}

#[derive(Debug, PartialEq)]
pub struct Foo {
    meta: Box<dyn MetaInstance>,
}

#[test]
fn main() {
    let foo1 = Foo { meta: Box::new(MetaImpl1(114)) };
    let foo2 = Foo { meta: Box::new(MetaImpl1(114)) };
    let foo3 = Foo { meta: Box::new(MetaImpl1(514)) };
    let foo4 = Foo { meta: Box::new(MetaImpl2(114)) };
    assert_eq!(foo1, foo2);
    assert_ne!(foo1, foo3);
    assert_ne!(foo1, foo4);
}
