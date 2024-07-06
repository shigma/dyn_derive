trait Meta<T>: ::dyn_std::any::Dyn {
    fn method_1(&self, v2: Box<dyn Meta<T>>, v3: &dyn Meta<T>);
    fn method_2(&self, v2: (T, Box<dyn Meta<T>>, &dyn Meta<T>));
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn method_1(arg1: Self, arg2: &Self);
    fn method_2(arg: (T, Self, &Self));
}
#[automatically_derived]
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, v2: Box<dyn Meta<T>>, v3: &dyn Meta<T>) {
        Factory::method_1(Self::downcast(v2), Self::downcast_ref(v3))
    }
    #[inline]
    fn method_2(&self, v2: (T, Box<dyn Meta<T>>, &dyn Meta<T>)) {
        Factory::method_2(
            match v2 {
                (v1, v2, v3) => (v1, Self::downcast(v2), Self::downcast_ref(v3)),
            },
        )
    }
}
