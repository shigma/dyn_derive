use dyn_derive::*;
use dyn_std::{Constructor, Instance};

#[dyn_trait]
pub trait Meta<T>: Clone {
    fn method_1(arg1: T, arg2: Self, arg3: &Self, arg4: Box<Self>) -> Self;
    fn method_3(arg1: Vec<Self>, arg2: Vec<&Self>) -> bool;
    fn method_4(arg1: Option<Self>, arg2: Option<&Self>) -> Option<Self>;
    fn method_5(arg1: Result<Self, ()>, arg2: Result<(), &Self>) -> Self;
    fn method_6(&self, arg1: &dyn Fn(Self) -> Self) -> Self;
    fn method_7(&self, arg1: Vec<Box<dyn FnMut(Self) -> Self>>) -> Self;
}

#[derive(Clone)]
struct MetaImpl<T>(T);

impl<T: Clone + 'static> Meta<T> for MetaImpl<T> {
    fn method_1(arg1: T, _arg2: Self, _arg3: &Self, _arg4: Box<Self>) -> Self {
        MetaImpl(arg1)
    }

    fn method_3(arg1: Vec<Self>, arg2: Vec<&Self>) -> bool {
        arg1.len() == arg2.len()
    }

    fn method_4(arg1: Option<Self>, _arg2: Option<&Self>) -> Option<Self> {
        arg1
    }

    fn method_5(arg1: Result<Self, ()>, _arg2: Result<(), &Self>) -> Self {
        arg1.unwrap()
    }

    fn method_6(&self, arg1: &dyn Fn(Self) -> Self) -> Self {
        arg1(self.clone())
    }

    fn method_7(&self, mut arg1: Vec<Box<dyn FnMut(Self) -> Self>>) -> Self {
        arg1[0](self.clone())
    }
}

#[test]
fn main() {
    let cons: Box<dyn MetaConstructor<i32>> = Box::new(Constructor::<MetaImpl<i32>>::new());
    let inst: Box<dyn MetaInstance<i32>> = Box::new(Instance::new(MetaImpl(42)));
    cons.method_1(0, inst.clone(), inst.as_ref(), inst.clone());
    cons.method_3(vec![inst.clone()], vec![inst.as_ref()]);
    cons.method_4(Some(inst.clone()), Some(inst.as_ref()));
    cons.method_5(Ok(inst.clone()), Err(inst.as_ref()));
    inst.method_6(&mut |x| x);
}
