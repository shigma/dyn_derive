use core::marker::PhantomData;

use crate::Dyn;

pub struct Instance<T, U>(pub T, PhantomData<U>);

impl<T, U> Instance<T, U> {
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: 'static, U: 'static> Instance<T, U> {
    #[inline]
    pub fn downcast_ref<D: Dyn + ?Sized>(v: &D) -> &T {
        &v.as_any().downcast_ref::<Self>().unwrap().0
    }

    #[inline]
    pub fn downcast_mut<D: Dyn + ?Sized>(v: &mut D) -> &mut T {
        &mut v.as_any_mut().downcast_mut::<Self>().unwrap().0
    }

    #[inline]
    pub fn downcast<D: Dyn + ?Sized>(v: Box<D>) -> T {
        v.as_any_box().downcast::<Self>().unwrap().0
    }
}

#[doc(hidden)]
pub struct Constructor<T, U>(PhantomData<(T, U)>);

impl<T, U> Constructor<T, U> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T, U> Default for Constructor<T, U> {
    fn default() -> Self {
        Self::new()
    }
}
