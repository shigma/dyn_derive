use core::any::Any;
use core::cmp;

pub trait PartialEq: Any {
    fn dyn_eq(&self, other: &dyn Any) -> bool;
}

impl<T: Any + cmp::PartialEq> PartialEq for T {
    #[inline]
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |other| self.eq(other))
    }
}

pub trait PartialOrd: Any {
    fn dyn_partial_cmp(&self, other: &dyn Any) -> Option<cmp::Ordering>;
}

impl<T: Any + cmp::PartialOrd> PartialOrd for T {
    #[inline]
    fn dyn_partial_cmp(&self, other: &dyn Any) -> Option<cmp::Ordering> {
        other.downcast_ref::<Self>().map_or(None, |other| self.partial_cmp(other))
    }
}
