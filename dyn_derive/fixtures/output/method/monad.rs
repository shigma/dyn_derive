trait Meta<T>: ::dyn_std::any::Dyn {
    fn option(&self, arg: Option<Box<dyn Meta<T>>>);
    fn result_1(&self, arg: Result<Box<dyn Meta<T>>, ()>);
    fn result_2(&self, arg: Result<(), Box<dyn Meta<T>>>);
    fn vec(&self, arg: Vec<Box<dyn Meta<T>>>);
    fn nested(&self, arg: Vec<(Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>)>);
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn option(arg: Option<Self>);
    fn result_1(arg: Result<Self, ()>);
    fn result_2(arg: Result<(), Self>);
    fn vec(arg: Vec<Self>);
    fn nested(arg: Vec<(Self, Option<Option<Self>>)>);
}
#[automatically_derived]
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn option(&self, a1: Option<Box<dyn Meta<T>>>) {
        let a1 = ::dyn_std::map::Map1::map(
            a1,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
        );
        Factory::option(a1)
    }
    #[inline]
    fn result_1(&self, a1: Result<Box<dyn Meta<T>>, ()>) {
        let a1 = ::dyn_std::map::Map2::map(
            a1,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
            |x: ()| x,
        );
        Factory::result_1(a1)
    }
    #[inline]
    fn result_2(&self, a1: Result<(), Box<dyn Meta<T>>>) {
        let a1 = ::dyn_std::map::Map2::map(
            a1,
            |x: ()| x,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
        );
        Factory::result_2(a1)
    }
    #[inline]
    fn vec(&self, a1: Vec<Box<dyn Meta<T>>>) {
        let a1 = ::dyn_std::map::Map1::map(
            a1,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
        );
        Factory::vec(a1)
    }
    #[inline]
    fn nested(&self, a1: Vec<(Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>)>) {
        let a1 = ::dyn_std::map::Map1::map(
            a1,
            |
                (a1, a2): (Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>),
            | -> (Factory, Option<Option<Factory>>) {
                let a1 = Self::downcast(a1);
                let a2 = ::dyn_std::map::Map1::map(
                    a2,
                    |x: Option<Box<dyn Meta<T>>>| -> Option<Factory> {
                        ::dyn_std::map::Map1::map(
                            x,
                            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
                        )
                    },
                );
                (a1, a2)
            },
        );
        Factory::nested(a1)
    }
}
