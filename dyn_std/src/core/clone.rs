use crate::Instance;

/// Dynamic (object-safe) version of [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html).
pub trait Clone {
    fn dyn_clone(&self) -> *mut ();
}

impl<T: core::clone::Clone> Clone for T {
    #[inline]
    fn dyn_clone(&self) -> *mut () {
        Box::<T>::into_raw(Box::new(self.clone())) as *mut ()
    }
}

impl Clone for str {
    #[inline]
    fn dyn_clone(&self) -> *mut () {
        Box::<str>::into_raw(Box::from(self)) as *mut ()
    }
}

impl<T: core::clone::Clone> Clone for [T] {
    #[inline]
    fn dyn_clone(&self) -> *mut () {
        Box::<[T]>::into_raw(self.iter().cloned().collect()) as *mut ()
    }
}

impl<T: core::clone::Clone> core::clone::Clone for Instance<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}
