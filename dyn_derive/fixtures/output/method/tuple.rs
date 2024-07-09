trait Meta<T>: ::dyn_std::any::Dyn {
    fn tuple_1(&self, arg1: ((Box<dyn Meta<T>>, T), Box<dyn Meta<T>>), arg2: T);
    fn tuple_2(&self) -> (T, (Box<dyn Meta<T>>, (T, Box<dyn Meta<T>>)));
    fn tuple_3(&self, arg: (T, Option<Box<dyn Meta<T>>>)) -> Vec<(Box<dyn Meta<T>>, T)>;
    fn tuple_4(&self, arg: (T, Vec<T>)) -> (Option<T>, T);
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn tuple_1(arg1: ((Self, T), Self), arg2: T);
    fn tuple_2() -> (T, (Self, (T, Self)));
    fn tuple_3(arg: (T, Option<Self>)) -> Vec<(Self, T)>;
    fn tuple_4(arg: (T, Vec<T>)) -> (Option<T>, T);
}
#[automatically_derived]
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn tuple_1(&self, ((v1, v2), v3): ((Box<dyn Meta<T>>, T), Box<dyn Meta<T>>), v4: T) {
        let v1 = Self::downcast(v1);
        let v3 = Self::downcast(v3);
        Factory::tuple_1(((v1, v2), v3), v4)
    }
    #[inline]
    fn tuple_2(&self) -> (T, (Box<dyn Meta<T>>, (T, Box<dyn Meta<T>>))) {
        let (v1, (v2, (v3, v4))) = Factory::tuple_2();
        let v2 = Box::new(::dyn_std::Instance::new(v2));
        let v4 = Box::new(::dyn_std::Instance::new(v4));
        (v1, (v2, (v3, v4)))
    }
    #[inline]
    fn tuple_3(
        &self,
        (v1, v2): (T, Option<Box<dyn Meta<T>>>),
    ) -> Vec<(Box<dyn Meta<T>>, T)> {
        let v2 = ::dyn_std::map::Map1::map(
            v2,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
        );
        ::dyn_std::map::Map1::map(
            Factory::tuple_3((v1, v2)),
            |(v3, v4): (Factory, T)| -> (Box<dyn Meta<T>>, T) {
                let v3 = Box::new(::dyn_std::Instance::new(v3));
                (v3, v4)
            },
        )
    }
    #[inline]
    fn tuple_4(&self, v1: (T, Vec<T>)) -> (Option<T>, T) {
        Factory::tuple_4(v1)
    }
}
