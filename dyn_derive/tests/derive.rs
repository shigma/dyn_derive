use dyn_traits::*;

use std::fmt::Debug;

#[macro_use]
extern crate dyn_derive;

pub trait Meta: Debug + DynPartialEq + DynClone + DynAdd {}

impl PartialEq for Box<dyn Meta> {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_any())
    }
}

impl Clone for Box<dyn Meta> {
    fn clone(&self) -> Self {
        ptr::convert_to_box(self, DynClone::dyn_clone)
    }
}

#[test]
fn derive() {

    #[derive(Debug, PartialEq, Clone)]
    pub struct MetaImpl {
        name: String,
    }

    impl Meta for MetaImpl {}
    
    impl std::ops::Add for MetaImpl {
        type Output = MetaImpl;
    
        fn add(self, other: MetaImpl) -> MetaImpl {
            MetaImpl {
                name: format!("{}{}", self.name, other.name),
            }
        }
    }


    #[derive(Debug, PartialEq, Clone)]
    pub struct Bar {
        meta: Box<dyn Meta>,
    }

    impl std::ops::Add for Bar {
        type Output = Bar;
    
        fn add(self, other: Bar) -> Bar {
            Bar {
                meta: dyn_traits::ptr::convert_into_box(self.meta, |m| m.dyn_add(other.meta.as_any_box())),
            }
        }
    }


    let bar1 = Bar { meta: Box::new(MetaImpl { name: "foo".into() }) };
    let bar2 = Bar { meta: Box::new(MetaImpl { name: "bar".into() }) };
    assert_eq!(bar1, Bar { meta: Box::new(MetaImpl { name: "foo".into() }) });
    let bar3 = bar1 + bar2;
    println!("{:?}", bar3);
    // assert_eq!(bar3.foo.magic(), 42);
}
