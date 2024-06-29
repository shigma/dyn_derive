use core::any::Any;

use crate::Dyn;
use crate::inst::Instance;

pub trait PartialEq: Dyn {
    fn dyn_eq(&self, other: &dyn Any) -> bool;
}

impl<T: Dyn + core::cmp::PartialEq> PartialEq for T {
    #[inline]
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |other| self.eq(other))
    }
}

impl<T: core::cmp::PartialEq, U> core::cmp::PartialEq for Instance<T, U> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

pub trait PartialOrd: Dyn {
    fn dyn_partial_cmp(&self, other: &dyn Any) -> Option<core::cmp::Ordering>;
}

impl<T: Dyn + core::cmp::PartialOrd> PartialOrd for T {
    #[inline]
    fn dyn_partial_cmp(&self, other: &dyn Any) -> Option<core::cmp::Ordering> {
        other.downcast_ref::<Self>().map_or(None, |other| self.partial_cmp(other))
    }
}

impl<T: core::cmp::PartialOrd, U> core::cmp::PartialOrd for Instance<T, U> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
