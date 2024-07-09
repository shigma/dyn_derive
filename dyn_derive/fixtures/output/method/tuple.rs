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
    fn tuple_1(&self, ((a1, a2), a3): ((Box<dyn Meta<T>>, T), Box<dyn Meta<T>>), a4: T) {
        let a1 = Self::downcast(a1);
        let a3 = Self::downcast(a3);
        Factory::tuple_1(((a1, a2), a3), a4)
    }
    #[inline]
    fn tuple_2(&self) -> (T, (Box<dyn Meta<T>>, (T, Box<dyn Meta<T>>))) {
        let (a1, (a2, (a3, a4))) = Factory::tuple_2();
        let a2 = Box::new(::dyn_std::Instance::new(a2));
        let a4 = Box::new(::dyn_std::Instance::new(a4));
        (a1, (a2, (a3, a4)))
    }
    #[inline]
    fn tuple_3(
        &self,
        (a1, a2): (T, Option<Box<dyn Meta<T>>>),
    ) -> Vec<(Box<dyn Meta<T>>, T)> {
        let a2 = ::dyn_std::map::Map1::map(
            a2,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
        );
        ::dyn_std::map::Map1::map(
            Factory::tuple_3((a1, a2)),
            |(a3, a4): (Factory, T)| -> (Box<dyn Meta<T>>, T) {
                let a3 = Box::new(::dyn_std::Instance::new(a3));
                (a3, a4)
            },
        )
    }
    #[inline]
    fn tuple_4(&self, a1: (T, Vec<T>)) -> (Option<T>, T) {
        Factory::tuple_4(a1)
    }
}
