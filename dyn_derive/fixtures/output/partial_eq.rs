trait Meta: ::dyn_std::cmp::PartialEq + ::dyn_std::any::Dyn {}

impl std::cmp::PartialEq for dyn Meta {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_any())
    }
}

impl std::cmp::PartialEq<&Self> for Box<dyn Meta> {
    #[inline]
    fn eq(&self, other: &&Self) -> bool {
        self.dyn_eq(other.as_any())
    }
}

trait MetaFactory: PartialEq + Sized + 'static {}

impl<Factory: MetaFactory> Meta for ::dyn_std::Instance<Factory, ()> {}
