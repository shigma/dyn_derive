use std::fmt::Debug;

#[macro_use]
extern crate dynex;

#[dyn_trait]
pub trait Meta: Debug + PartialEq + Clone + Add {}

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


    #[derive(Debug, Clone, PartialEqFix)]
    pub struct Bar {
        meta: Box<dyn Meta>,
    }

    impl std::ops::Add for Bar {
        type Output = Bar;
    
        fn add(self, other: Bar) -> Bar {
            Bar {
                meta: self.meta + other.meta,
            }
        }
    }


    let bar1 = Bar { meta: Box::new(MetaImpl { name: "foo".into() }) };
    let bar2 = Bar { meta: Box::new(MetaImpl { name: "bar".into() }) };
    assert_eq!(bar1, Bar { meta: Box::new(MetaImpl { name: "foo".into() }) });
    let bar3 = bar1 + bar2;
    println!("{:?}", bar3);
}
