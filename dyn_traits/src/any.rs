use core::any::Any;

/// This trait is an extension trait to [`Any`], and adds methods to retrieve a `&dyn Any`
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

/// This is a shim around `Dyn` to avoid some boilerplate code.
/// It is a separate trait because it is also implemented
/// on runtime polymorphic traits (which are `!Sized`).
pub trait Downcast: Dyn {
    /// Returns `true` if the boxed type is the same as `T`.
    ///
    /// Forward to the method defined on the type `Any`.
    #[inline]
    fn is<T: Dyn>(&self) -> bool {
        self.as_any().is::<T>()
    }

    /// Forward to the method defined on the type `Any`.
    #[inline]
    fn downcast_ref<T: Dyn>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    /// Forward to the method defined on the type `Any`.
    #[inline]
    fn downcast_mut<T: Dyn>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}

impl<T: ?Sized + Dyn> Downcast for T {}
