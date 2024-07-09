trait Meta<T>: ::dyn_std::any::Dyn {
    fn method_1(&self, arg1: Box<dyn Meta<T>>, arg2: &dyn Meta<T>);
    fn method_2(&self, arg: (T, Box<dyn Meta<T>>, &dyn Meta<T>));
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn method_1(arg1: Self, arg2: &Self);
    fn method_2(arg: (T, Self, &Self));
}
#[automatically_derived]
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, a1: Box<dyn Meta<T>>, a2: &dyn Meta<T>) {
        let a1 = Self::downcast(a1);
        let a2 = Self::downcast_ref(a2);
        Factory::method_1(a1, a2)
    }
    #[inline]
    fn method_2(&self, (a1, a2, a3): (T, Box<dyn Meta<T>>, &dyn Meta<T>)) {
        let a2 = Self::downcast(a2);
        let a3 = Self::downcast_ref(a3);
        Factory::method_2((a1, a2, a3))
    }
}
