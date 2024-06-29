use core::marker::PhantomData;

pub struct Instance<T, U>(pub T, PhantomData<U>);

impl<T, U> Instance<T, U> {
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

// `Deref` and `DerefMut` are not implemented for `Instance<T, U>,`
// because they may be confused with methods from `dyn Trait`.
