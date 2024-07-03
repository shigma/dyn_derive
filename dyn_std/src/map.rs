use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Help traits for unsized coercion.

macro_rules! map_trait {
    ($n:ident; $($s:ident),*; $($t:ident),*; $($f:ident),*) => {
        pub trait $n<$($t),*> {
            type Input<$($s),*>;
            fn map<$($s),*>(value: Self::Input<$($s),*>, $($f: fn($s) -> $t),*) -> Self;
        }
    };
}

map_trait!(Map1; S1; T1; f1);
map_trait!(Map2; S1, S2; T1, T2; f1, f2);
map_trait!(Map3; S1, S2, S3; T1, T2, T3; f1, f2, f3);
map_trait!(Map4; S1, S2, S3, S4; T1, T2, T3, T4; f1, f2, f3, f4);
map_trait!(Map5; S1, S2, S3, S4, S5; T1, T2, T3, T4, T5; f1, f2, f3, f4, f5);
map_trait!(Map6; S1, S2, S3, S4, S5, S6; T1, T2, T3, T4, T5, T6; f1, f2, f3, f4, f5, f6);

impl<T1> Map1<T1> for Option<T1> {
    type Input<S1> = Option<S1>;
    #[inline]
    fn map<S1>(value: Self::Input<S1>, f1: fn(S1) -> T1) -> Self {
        value.map(f1)
    }
}

impl<T1> Map1<T1> for Vec<T1> {
    type Input<S1> = Vec<S1>;
    #[inline]
    fn map<S1>(value: Self::Input<S1>, f1: fn(S1) -> T1) -> Self {
        value.into_iter().map(f1).collect()
    }
}

impl<T1: Eq + Hash> Map1<T1> for HashSet<T1> {
    type Input<S1> = HashSet<S1>;
    #[inline]
    fn map<S1>(value: Self::Input<S1>, f1: fn(S1) -> T1) -> Self {
        value.into_iter().map(f1).collect()
    }
}

impl<T1, T2> Map2<T1, T2> for Result<T1, T2> {
    type Input<S1, S2> = Result<S1, S2>;
    #[inline]
    fn map<S1, S2>(value: Self::Input<S1, S2>, f1: fn(S1) -> T1, f2: fn(S2) -> T2) -> Self {
        value.map(f1).map_err(f2)
    }
}

impl<T1: Eq + Hash, T2> Map2<T1, T2> for HashMap<T1, T2> {
    type Input<S1, S2> = HashMap<S1, S2>;
    #[inline]
    fn map<S1, S2>(value: Self::Input<S1, S2>, f1: fn(S1) -> T1, f2: fn(S2) -> T2) -> Self {
        value.into_iter().map(|(k, v)| (f1(k), f2(v))).collect()
    }
}
