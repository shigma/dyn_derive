pub trait Clone {
    fn dyn_clone(&self) -> *mut ();
}

impl<T: std::clone::Clone> Clone for T {
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

impl<T: std::clone::Clone> Clone for [T] {
    #[inline]
    fn dyn_clone(&self) -> *mut () {
        Box::<[T]>::into_raw(self.iter().cloned().collect()) as *mut ()
    }
}
