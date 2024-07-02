use core::any::Any;

use crate::{Dyn, Instance};

/// Dynamic (object-safe) version of [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html).
pub trait PartialEq {
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

/// Dynamic (object-safe) version of [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html).
pub trait Eq: PartialEq {}

impl<T: Dyn + core::cmp::Eq> Eq for T {}

impl<T: core::cmp::Eq, U> core::cmp::Eq for Instance<T, U> {}

/// Dynamic (object-safe) version of [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html).
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

/// Dynamic (object-safe) version of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html).
pub trait Ord: PartialOrd {
    fn dyn_cmp(&self, other: &dyn Any) -> core::cmp::Ordering;
}

impl<T: Dyn + core::cmp::Ord> Ord for T {
    #[inline]
    fn dyn_cmp(&self, other: &dyn Any) -> core::cmp::Ordering {
        self.cmp(other.downcast_ref::<Self>().unwrap())
    }
}

impl<T: core::cmp::Ord, U> core::cmp::Ord for Instance<T, U> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
