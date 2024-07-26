trait MetaInstance<T>: ::dyn_std::any::Dyn {
    fn method_1(&self, arg1: Box<dyn MetaInstance<T>>, arg2: &dyn MetaInstance<T>);
    fn method_2(&self, arg: (T, Box<dyn MetaInstance<T>>, &dyn MetaInstance<T>));
}
trait Meta<T: 'static>: Sized + 'static {
    fn method_1(arg1: Self, arg2: &Self);
    fn method_2(arg: (T, Self, &Self));
}
#[automatically_derived]
impl<T: 'static, Factory: Meta<T>> MetaInstance<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, a1: Box<dyn MetaInstance<T>>, a2: &dyn MetaInstance<T>) {
        let a1 = Self::downcast(a1);
        let a2 = Self::downcast_ref(a2);
        Factory::method_1(a1, a2)
    }
    #[inline]
    fn method_2(
        &self,
        (a1, a2, a3): (T, Box<dyn MetaInstance<T>>, &dyn MetaInstance<T>),
    ) {
        let a2 = Self::downcast(a2);
        let a3 = Self::downcast_ref(a3);
        Factory::method_2((a1, a2, a3))
    }
}
