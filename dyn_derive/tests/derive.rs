use dyn_traits::*;

#[macro_use]
extern crate dyn_derive;

#[test]
fn derive_partial_eq() {
    pub trait Foo: std::fmt::Debug + DynPartialEq + DynClone {
        fn foo(&self) -> isize {
            42
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FooImpl;

    impl Foo for FooImpl {}

    #[derive(DynPartialEq)]
    pub struct Bar {
        foo: Box<dyn Foo>,
    }

    let bar1 = Bar { foo: Box::new(FooImpl) };
    let bar2 = Bar { foo: Box::new(FooImpl) };
    bar1 == bar2;
}
