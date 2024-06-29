macro_rules! map_trait {
    ($n:ident; $($s:ident),*; $($t:ident),*; $($f:ident),*) => {
        pub trait $n<$($s),*> {
            type Output<$($t),*>;
            fn map<$($t),*>(self, $($f: fn($s) -> $t),*) -> Self::Output<$($t),*>;
        }
    };
}

map_trait!(Map1; S1; T1; f1);
map_trait!(Map2; S1, S2; T1, T2; f1, f2);
map_trait!(Map3; S1, S2, S3; T1, T2, T3; f1, f2, f3);
map_trait!(Map4; S1, S2, S3, S4; T1, T2, T3, T4; f1, f2, f3, f4);
map_trait!(Map5; S1, S2, S3, S4, S5; T1, T2, T3, T4, T5; f1, f2, f3, f4, f5);
map_trait!(Map6; S1, S2, S3, S4, S5, S6; T1, T2, T3, T4, T5, T6; f1, f2, f3, f4, f5, f6);

impl<S1> Map1<S1> for Option<S1> {
    type Output<T1> = Option<T1>;
    #[inline]
    fn map<T1>(self, f1: fn(S1) -> T1) -> Self::Output<T1> {
        self.map(f1)
    }
}

impl<S1, S2> Map2<S1, S2> for Result<S1, S2> {
    type Output<T1, T2> = Result<T1, T2>;
    #[inline]
    fn map<T1, T2>(self, f1: fn(S1) -> T1, f2: fn(S2) -> T2) -> Self::Output<T1, T2> {
        self.map(f1).map_err(f2)
    }
}
