use core::marker::PhantomData;

use crate::Dyn;

pub struct Instance<T>(pub T);

impl<T> Instance<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: 'static> Instance<T> {
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
pub struct Constructor<T>(PhantomData<T>);

impl<T> Constructor<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Constructor<T> {
    fn default() -> Self {
        Self::new()
    }
}
