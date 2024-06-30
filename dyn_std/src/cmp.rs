use std::any::Any;

use crate::Dyn;
use crate::inst::Instance;

/// Dynamic (object-safe) version of [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html).
pub trait PartialEq {
    fn dyn_eq(&self, other: &dyn Any) -> bool;
}

impl<T: Dyn + std::cmp::PartialEq> PartialEq for T {
    #[inline]
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |other| self.eq(other))
    }
}

impl<T: std::cmp::PartialEq, U> std::cmp::PartialEq for Instance<T, U> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

/// Dynamic (object-safe) version of [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html).
pub trait Eq: PartialEq {}

impl<T: Dyn + std::cmp::Eq> Eq for T {}

impl<T: std::cmp::Eq, U> std::cmp::Eq for Instance<T, U> {}

/// Dynamic (object-safe) version of [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html).
pub trait PartialOrd: Dyn {
    fn dyn_partial_cmp(&self, other: &dyn Any) -> Option<std::cmp::Ordering>;
}

impl<T: Dyn + std::cmp::PartialOrd> PartialOrd for T {
    #[inline]
    fn dyn_partial_cmp(&self, other: &dyn Any) -> Option<std::cmp::Ordering> {
        other.downcast_ref::<Self>().map_or(None, |other| self.partial_cmp(other))
    }
}

impl<T: std::cmp::PartialOrd, U> std::cmp::PartialOrd for Instance<T, U> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// Dynamic (object-safe) version of [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html).
pub trait Ord: PartialOrd {
    fn dyn_cmp(&self, other: &dyn Any) -> std::cmp::Ordering;
}

impl<T: Dyn + std::cmp::Ord> Ord for T {
    #[inline]
    fn dyn_cmp(&self, other: &dyn Any) -> std::cmp::Ordering {
        self.cmp(other.downcast_ref::<Self>().unwrap())
    }
}

impl<T: std::cmp::Ord, U> std::cmp::Ord for Instance<T, U> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
