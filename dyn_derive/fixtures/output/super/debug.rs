trait Meta: Debug + ::dyn_std::any::Dyn {}
trait MetaFactory: Debug + Sized + 'static {}
#[automatically_derived]
impl<Factory: MetaFactory> Meta for ::dyn_std::Instance<Factory, ()> {}
