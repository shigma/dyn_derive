use core::any::Any;

macro_rules! unary {
    ($trait:ident, $method:ident, $original:ident) => {
        pub trait $trait: Any {
            fn $method(self: Box<Self>) -> *mut ();
        }

        impl<T: Any + core::ops::$trait<Output = T>> $trait for T {
            #[inline]
            fn $method(self: Box<Self>) -> *mut () {
                Box::leak(Box::from((*self).$original())) as *const T as *mut ()
            }
        }
    };
}

macro_rules! binary {
    ($trait:ident, $method:ident, $original:ident) => {
        pub trait $trait: Any {
            fn $method(self: Box<Self>, other: Box<dyn Any>) -> *mut ();
        }

        impl<T: Any + core::ops::$trait<Output = T>> $trait for T {
            #[inline]
            fn $method(self: Box<Self>, other: Box<dyn Any>) -> *mut () {
                let other = other.downcast::<Self>().unwrap();
                Box::leak(Box::from((*self).$original(*other))) as *const T as *mut ()
            }
        }
    };
}

macro_rules! assign {
    ($trait:ident, $method:ident, $original:ident) => {
        pub trait $trait: Any {
            fn $method(&mut self, other: Box<dyn Any>);
        }

        impl<T: Any + core::ops::$trait> $trait for T {
            #[inline]
            fn $method(&mut self, other: Box<dyn Any>) {
                let other = other.downcast::<T>().unwrap();
                self.$original(*other);
            }
        }
    };
}

unary!(Neg, dyn_neg, neg);
unary!(Not, dyn_not, not);

binary!(Add, dyn_add, add);
binary!(Sub, dyn_sub, sub);
binary!(Mul, dyn_mul, mul);
binary!(Div, dyn_div, div);
binary!(Rem, dyn_rem, rem);
binary!(BitAnd, dyn_bitand, bitand);
binary!(BitOr, dyn_bitor, bitor);
binary!(BitXor, dyn_bitxor, bitxor);
binary!(Shl, dyn_shl, shl);
binary!(Shr, dyn_shr, shr);

assign!(AddAssign, dyn_add_assign, add_assign);
assign!(SubAssign, dyn_sub_assign, sub_assign);
assign!(MulAssign, dyn_mul_assign, mul_assign);
assign!(DivAssign, dyn_div_assign, div_assign);
assign!(RemAssign, dyn_rem_assign, rem_assign);
assign!(BitAndAssign, dyn_bitand_assign, bitand_assign);
assign!(BitOrAssign, dyn_bitor_assign, bitor_assign);
assign!(BitXorAssign, dyn_bitxor_assign, bitxor_assign);
assign!(ShlAssign, dyn_shl_assign, shl_assign);
assign!(ShrAssign, dyn_shr_assign, shr_assign);
