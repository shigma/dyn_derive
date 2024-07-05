use core::any::Any;

/// This trait is the base trait for most of `dyn_std` traits,
/// and adds methods to retrieve a `&dyn Any`.
pub trait Dyn: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any> Dyn for T {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

#[doc(hidden)]
pub trait Fat<T: ?Sized>: AsRef<T> + Sized {
    fn into_box(self, f: impl FnOnce(Self) -> *mut ()) -> Box<T> {
        let mut fat_ptr = self.as_ref() as *const T;
        let data_ptr = &mut fat_ptr as *mut *const T as *mut *mut ();
        unsafe {
            *data_ptr = f(self);
            Box::from_raw(fat_ptr as *mut T)
        }
    }

    fn to_box(self, f: impl FnOnce(&T) -> *mut ()) -> Box<T> {
        let mut fat_ptr = self.as_ref() as *const T;
        let data_ptr = &mut fat_ptr as *mut *const T as *mut *mut ();
        unsafe {
            *data_ptr = f(self.as_ref());
            Box::from_raw(fat_ptr as *mut T)
        }
    }
}

impl<T: ?Sized, R: AsRef<T>> Fat<T> for R {}
