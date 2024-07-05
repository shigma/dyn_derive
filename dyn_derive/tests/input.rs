use dyn_derive::*;
use dyn_std::Instance;

#[dyn_trait]
pub trait Meta<T>: Clone {
    fn method_1(arg1: T, arg2: Self, arg3: &Self, arg4: Box<Self>) -> Self;
    fn method_3(arg1: Vec<Self>, arg2: Vec<&Self>) -> Self;
    fn method_4(arg1: Option<Self>, arg2: Option<&Self>) -> Self;
    fn method_5(arg1: Result<Self, ()>, arg2: Result<(), &Self>) -> Self;
}

#[derive(Clone)]
struct MetaImpl<T>(T);

impl<T: Clone + 'static> MetaFactory<T> for MetaImpl<T> {
    fn method_1(arg1: T, _arg2: Self, _arg3: &Self, _arg4: Box<Self>) -> Self {
        MetaImpl(arg1)
    }

    fn method_3(mut arg1: Vec<Self>, _arg2: Vec<&Self>) -> Self {
        arg1.pop().unwrap()
    }

    fn method_4(arg1: Option<Self>, _arg2: Option<&Self>) -> Self {
        arg1.unwrap()
    }

    fn method_5(arg1: Result<Self, ()>, _arg2: Result<(), &Self>) -> Self {
        arg1.unwrap()
    }
}

#[test]
fn main() {
    let instance: Box<dyn Meta<i32>> = Box::new(Instance::new(MetaImpl(42)));
    let _1 = instance.method_1(0, instance.clone(), instance.as_ref(), Box::new(instance.clone()));
    let _3 = instance.method_3(vec![instance.clone()], vec![instance.as_ref()]);
    let _4 = instance.method_4(Some(instance.clone()), Some(instance.as_ref()));
    let _5 = instance.method_5(Ok(instance.clone()), Err(instance.as_ref()));
}
