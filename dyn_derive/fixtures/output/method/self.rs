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
    fn method_1(&self, v1: Box<dyn Meta<T>>, v2: &dyn Meta<T>) {
        let v1 = Self::downcast(v1);
        let v2 = Self::downcast_ref(v2);
        Factory::method_1(v1, v2)
    }
    #[inline]
    fn method_2(&self, (v1, v2, v3): (T, Box<dyn Meta<T>>, &dyn Meta<T>)) {
        let v2 = Self::downcast(v2);
        let v3 = Self::downcast_ref(v3);
        Factory::method_2((v1, v2, v3))
    }
}
