use dyn_derive::*;
use dyn_std::Instance;

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
    let instance: Box<dyn MetaInstance<i32>> = Box::new(Instance::new(MetaImpl(42)));
    instance.method_1(0, instance.clone(), instance.as_ref(), instance.clone());
    instance.method_3(vec![instance.clone()], vec![instance.as_ref()]);
    instance.method_4(Some(instance.clone()), Some(instance.as_ref()));
    instance.method_5(Ok(instance.clone()), Err(instance.as_ref()));
    instance.method_6(&mut |x| x);
}
