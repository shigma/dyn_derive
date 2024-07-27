trait Meta: Debug + Sized + 'static {}
trait MetaInstance: Debug + ::dyn_std::any::Dyn {}
trait MetaConstructor {}
#[automatically_derived]
impl<Factory: Meta> MetaInstance for ::dyn_std::Instance<Factory, ()> {}
#[automatically_derived]
impl<Factory: Meta> MetaConstructor for ::dyn_std::Constructor<Factory, ()> {}
