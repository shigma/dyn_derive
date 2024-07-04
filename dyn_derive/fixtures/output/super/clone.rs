trait Meta: ::dyn_std::clone::Clone + ::dyn_std::any::Dyn {}
impl Clone for Box<dyn Meta> {
    #[inline]
    fn clone(&self) -> Self {
        ::dyn_std::Fat::to_box(self, ::dyn_std::clone::Clone::dyn_clone)
    }
}
trait MetaFactory: Clone + Sized + 'static {}
impl<Factory: MetaFactory> Meta for ::dyn_std::Instance<Factory, ()> {}
