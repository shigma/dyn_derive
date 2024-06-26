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

pub fn convert_into_box<T: ?Sized, R: AsRef<T>>(t: R, f: impl FnOnce(R) -> *mut ()) -> Box<T> {
    let mut fat_ptr = t.as_ref() as *const T;
    let data_ptr = &mut fat_ptr as *mut *const T as *mut *mut ();
    unsafe {
        *data_ptr = f(t);
        Box::from_raw(fat_ptr as *mut T)
    }
}

pub fn convert_to_box<T: ?Sized>(t: &T, f: impl FnOnce(&T) -> *mut ()) -> Box<T> {
    let mut fat_ptr = t as *const T;
    let data_ptr = &mut fat_ptr as *mut *const T as *mut *mut ();
    unsafe {
        *data_ptr = f(t);
        Box::from_raw(fat_ptr as *mut T)
    }
}
