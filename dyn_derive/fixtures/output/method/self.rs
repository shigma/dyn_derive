trait Meta<T>: ::dyn_std::any::Dyn {
    fn method_1(&self, v1: Box<dyn Meta<T>>, v2: &Box<dyn Meta<T>>);
    fn method_2(&self, v1: (T, Box<dyn Meta<T>>, &Box<dyn Meta<T>>));
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn method_1(arg1: Self, arg2: &Self);
    fn method_2(arg: (T, Self, &Self));
}
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, v1: Box<dyn Meta<T>>, v2: &Box<dyn Meta<T>>) {
        let v1 = Self::downcast(v1);
        let v2 = Self::downcast_ref(&**v2);
        <Factory as MetaFactory<T>>::method_1(v1, v2)
    }
    #[inline]
    fn method_2(&self, v1: (T, Box<dyn Meta<T>>, &Box<dyn Meta<T>>)) {
        let v1 = match v1 {
            (v1, v2, v3) => (v1, Self::downcast(v2), Self::downcast_ref(&**v3)),
        };
        <Factory as MetaFactory<T>>::method_2(v1)
    }
}
