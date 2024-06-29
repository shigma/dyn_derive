pub trait Map1<S1, T1> {
    type Output;
    fn map1(self, f1: fn(S1) -> T1) -> Self::Output;
}

impl<S1, T1> Map1<S1, T1> for Option<S1> {
    type Output = Option<T1>;
    #[inline]
    fn map1(self, f1: fn(S1) -> T1) -> Self::Output {
        self.map(f1)
    }
}

pub trait Map2<S1, T1, S2, T2> {
    type Output;
    fn map2(self, f1: fn(S1) -> T1, f2: fn(S2) -> T2) -> Self::Output;
}

impl<S1, T1, S2, T2> Map2<S1, T1, S2, T2> for Result<S1, S2> {
    type Output = Result<T1, T2>;
    #[inline]
    fn map2(self, f1: fn(S1) -> T1, f2: fn(S2) -> T2) -> Self::Output {
        self.map(f1).map_err(f2)
    }
}
