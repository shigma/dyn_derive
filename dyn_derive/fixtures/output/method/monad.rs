trait Meta<T>: ::dyn_std::any::Dyn {
    fn option(&self, v2: Option<Box<dyn Meta<T>>>);
    fn result_1(&self, v2: Result<Box<dyn Meta<T>>, ()>);
    fn result_2(&self, v2: Result<(), Box<dyn Meta<T>>>);
    fn vec(&self, v2: Vec<Box<dyn Meta<T>>>);
    fn nested(&self, v2: Vec<(Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>)>);
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
    fn option(&self, v2: Option<Box<dyn Meta<T>>>) {
        Factory::option(
            ::dyn_std::map::Map1::map(
                v2,
                |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
            ),
        )
    }
    #[inline]
    fn result_1(&self, v2: Result<Box<dyn Meta<T>>, ()>) {
        Factory::result_1(
            ::dyn_std::map::Map2::map(
                v2,
                |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
                |x: ()| -> () { x },
            ),
        )
    }
    #[inline]
    fn result_2(&self, v2: Result<(), Box<dyn Meta<T>>>) {
        Factory::result_2(
            ::dyn_std::map::Map2::map(
                v2,
                |x: ()| -> () { x },
                |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
            ),
        )
    }
    #[inline]
    fn vec(&self, v2: Vec<Box<dyn Meta<T>>>) {
        Factory::vec(
            ::dyn_std::map::Map1::map(
                v2,
                |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
            ),
        )
    }
    #[inline]
    fn nested(&self, v2: Vec<(Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>)>) {
        Factory::nested(
            ::dyn_std::map::Map1::map(
                v2,
                |
                    x: (Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>),
                | -> (Factory, Option<Option<Factory>>) {
                    match x {
                        (v1, v2) => {
                            (
                                Self::downcast(v1),
                                ::dyn_std::map::Map1::map(
                                    v2,
                                    |x: Option<Box<dyn Meta<T>>>| -> Option<Factory> {
                                        ::dyn_std::map::Map1::map(
                                            x,
                                            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
                                        )
                                    },
                                ),
                            )
                        }
                    }
                },
            ),
        )
    }
}
