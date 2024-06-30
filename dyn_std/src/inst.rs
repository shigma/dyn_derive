use core::marker::PhantomData;
use std::hash::{Hash, Hasher};

pub struct Instance<T, U>(pub T, PhantomData<U>);

impl<T, U> Instance<T, U> {
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: Hash, U> Hash for Instance<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
