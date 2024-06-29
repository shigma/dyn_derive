use std::marker::PhantomData;

pub struct Instance<T, U>(pub T, PhantomData<U>);

impl<T, U> Instance<T, U> {
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: core::fmt::Debug, U> core::fmt::Debug for Instance<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
