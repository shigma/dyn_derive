trait Meta: ::dyn_std::ops::Add + ::dyn_std::any::Dyn {}

impl std::ops::Add for Box<dyn Meta> {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        ::dyn_std::Fat::into_box(self, |m| m.dyn_add(other.as_any_box()))
    }
}

trait MetaFactory: Add<Output = Self> + Sized + 'static {}

impl<Factory: MetaFactory> Meta for ::dyn_std::Instance<Factory, ()> {}
