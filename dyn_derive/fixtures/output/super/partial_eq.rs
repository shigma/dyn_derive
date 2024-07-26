trait MetaInstance: ::dyn_std::cmp::PartialEq + ::dyn_std::any::Dyn {}
#[automatically_derived]
impl std::cmp::PartialEq for dyn MetaInstance {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_any())
    }
}
#[automatically_derived]
impl std::cmp::PartialEq<&Self> for Box<dyn MetaInstance> {
    #[inline]
    fn eq(&self, other: &&Self) -> bool {
        self.dyn_eq(other.as_any())
    }
}
trait Meta: PartialEq + Sized + 'static {}
#[automatically_derived]
impl<Factory: Meta> MetaInstance for ::dyn_std::Instance<Factory, ()> {}
