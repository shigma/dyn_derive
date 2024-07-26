trait MetaInstance: Debug + ::dyn_std::any::Dyn {}
trait Meta: Debug + Sized + 'static {}
#[automatically_derived]
impl<Factory: Meta> MetaInstance for ::dyn_std::Instance<Factory, ()> {}
