use core::any::Any;

use crate::{Dyn, Instance};

macro_rules! unary {
    ($trait:ident, $method:ident, $original:ident, $doc:tt) => {
        #[doc = $doc]
        pub trait $trait {
            fn $method(self: Box<Self>) -> *mut ();
        }

        impl<T: Dyn + core::ops::$trait<Output = T>> $trait for T {
            #[inline]
            fn $method(self: Box<Self>) -> *mut () {
                Box::leak(Box::from((*self).$original())) as *const T as *mut ()
            }
        }

        impl<T: core::ops::$trait<Output = T>> core::ops::$trait for Instance<T> {
            type Output = Self;
            #[inline]
            fn $original(self) -> Self {
                Self::new(self.0.$original())
            }
        }
    };
}

macro_rules! binary {
    ($trait:ident, $method:ident, $original:ident, $doc:tt) => {
        #[doc = $doc]
        pub trait $trait {
            fn $method(self: Box<Self>, other: Box<dyn Any>) -> *mut ();
        }

        impl<T: Dyn + core::ops::$trait<Output = T>> $trait for T {
            #[inline]
            fn $method(self: Box<Self>, other: Box<dyn Any>) -> *mut () {
                let other = other.downcast::<Self>().unwrap();
                Box::leak(Box::from((*self).$original(*other))) as *const T as *mut ()
            }
        }

        impl<T: core::ops::$trait<Output = T>> core::ops::$trait for Instance<T> {
            type Output = Self;
            #[inline]
            fn $original(self, other: Self) -> Self {
                Self::new(self.0.$original(other.0))
            }
        }
    };
}

macro_rules! assign {
    ($trait:ident, $method:ident, $original:ident, $doc:tt) => {
        #[doc = $doc]
        pub trait $trait {
            fn $method(&mut self, other: Box<dyn Any>);
        }

        impl<T: Dyn + core::ops::$trait> $trait for T {
            #[inline]
            fn $method(&mut self, other: Box<dyn Any>) {
                let other = other.downcast::<T>().unwrap();
                self.$original(*other);
            }
        }

        impl<T: core::ops::$trait> core::ops::$trait for Instance<T> {
            #[inline]
            fn $original(&mut self, other: Self) {
                self.0.$original(other.0)
            }
        }
    };
}

unary!(Neg, dyn_neg, neg, "Dynamic (object-safe) version of [`Neg`](https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html)");
unary!(Not, dyn_not, not, "Dynamic (object-safe) version of [`Not`](https://doc.rust-lang.org/nightly/core/ops/trait.Not.html)");

binary!(Add, dyn_add, add, "Dynamic (object-safe) version of [`Add`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html)");
binary!(Sub, dyn_sub, sub, "Dynamic (object-safe) version of [`Sub`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html)");
binary!(Mul, dyn_mul, mul, "Dynamic (object-safe) version of [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html)");
binary!(Div, dyn_div, div, "Dynamic (object-safe) version of [`Div`](https://doc.rust-lang.org/nightly/core/ops/trait.Div.html)");
binary!(Rem, dyn_rem, rem, "Dynamic (object-safe) version of [`Rem`](https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html)");
binary!(BitAnd, dyn_bitand, bitand, "Dynamic (object-safe) version of [`BitAnd`](https://doc.rust-lang.org/nightly/core/ops/trait.BitAnd.html)");
binary!(BitOr, dyn_bitor, bitor, "Dynamic (object-safe) version of [`BitOr`](https://doc.rust-lang.org/nightly/core/ops/trait.BitOr.html)");
binary!(BitXor, dyn_bitxor, bitxor, "Dynamic (object-safe) version of [`BitXor`](https://doc.rust-lang.org/nightly/core/ops/trait.BitXor.html)");
binary!(Shl, dyn_shl, shl, "Dynamic (object-safe) version of [`Shl`](https://doc.rust-lang.org/nightly/core/ops/trait.Shl.html)");
binary!(Shr, dyn_shr, shr, "Dynamic (object-safe) version of [`Shr`](https://doc.rust-lang.org/nightly/core/ops/trait.Shr.html)");

assign!(AddAssign, dyn_add_assign, add_assign, "Dynamic (object-safe) version of [`AddAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.AddAssign.html)");
assign!(SubAssign, dyn_sub_assign, sub_assign, "Dynamic (object-safe) version of [`SubAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.SubAssign.html)");
assign!(MulAssign, dyn_mul_assign, mul_assign, "Dynamic (object-safe) version of [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html)");
assign!(DivAssign, dyn_div_assign, div_assign, "Dynamic (object-safe) version of [`DivAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.DivAssign.html)");
assign!(RemAssign, dyn_rem_assign, rem_assign, "Dynamic (object-safe) version of [`RemAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.RemAssign.html)");
assign!(BitAndAssign, dyn_bitand_assign, bitand_assign, "Dynamic (object-safe) version of [`BitAndAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.BitAndAssign.html)");
assign!(BitOrAssign, dyn_bitor_assign, bitor_assign, "Dynamic (object-safe) version of [`BitOrAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.BitOrAssign.html)");
assign!(BitXorAssign, dyn_bitxor_assign, bitxor_assign, "Dynamic (object-safe) version of [`BitXorAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.BitXorAssign.html)");
assign!(ShlAssign, dyn_shl_assign, shl_assign, "Dynamic (object-safe) version of [`ShlAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShlAssign.html)");
assign!(ShrAssign, dyn_shr_assign, shr_assign, "Dynamic (object-safe) version of [`ShrAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShrAssign.html)");
