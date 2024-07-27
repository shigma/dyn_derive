trait Meta: Clone + Sized + 'static {}
trait MetaInstance: ::dyn_std::clone::Clone + ::dyn_std::any::Dyn {}
trait MetaConstructor {}
#[automatically_derived]
impl Clone for Box<dyn MetaInstance> {
    #[inline]
    fn clone(&self) -> Self {
        ::dyn_std::Fat::to_box(self, ::dyn_std::clone::Clone::dyn_clone)
    }
}
#[automatically_derived]
impl<Factory: Meta> MetaInstance for ::dyn_std::Instance<Factory> {}
#[automatically_derived]
impl<Factory: Meta> MetaConstructor for ::dyn_std::Constructor<Factory> {}
