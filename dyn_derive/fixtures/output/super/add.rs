trait Meta: Add<Output = Self> + Sized + 'static {}
trait MetaInstance: ::dyn_std::ops::Add + ::dyn_std::any::Dyn {}
trait MetaConstructor {}
#[automatically_derived]
impl std::ops::Add for Box<dyn MetaInstance> {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        ::dyn_std::Fat::into_box(self, |m| m.dyn_add(other.as_any_box()))
    }
}
#[automatically_derived]
impl<Factory: Meta> MetaInstance for ::dyn_std::Instance<Factory> {}
#[automatically_derived]
impl<Factory: Meta> MetaConstructor for ::dyn_std::Constructor<Factory> {}
