use core::marker::PhantomData;

pub struct Instance<T, U>(pub T, PhantomData<U>);

impl<T, U> Instance<T, U> {
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}
