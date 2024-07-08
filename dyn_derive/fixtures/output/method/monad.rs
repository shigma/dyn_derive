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
    fn option(&self, v1: Option<Box<dyn Meta<T>>>) {
        let v1 = ::dyn_std::map::Map1::map(
            v1,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
        );
        Factory::option(v1)
    }
    #[inline]
    fn result_1(&self, v1: Result<Box<dyn Meta<T>>, ()>) {
        let v1 = ::dyn_std::map::Map2::map(
            v1,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
            |x: ()| -> () {
                let () = x;
                ()
            },
        );
        Factory::result_1(v1)
    }
    #[inline]
    fn result_2(&self, v1: Result<(), Box<dyn Meta<T>>>) {
        let v1 = ::dyn_std::map::Map2::map(
            v1,
            |x: ()| -> () { x },
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
        );
        Factory::result_2(v1)
    }
    #[inline]
    fn vec(&self, v1: Vec<Box<dyn Meta<T>>>) {
        let v1 = ::dyn_std::map::Map1::map(
            v1,
            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
        );
        Factory::vec(v1)
    }
    #[inline]
    fn nested(&self, v1: Vec<(Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>)>) {
        let v1 = ::dyn_std::map::Map1::map(
            v1,
            |
                x: (Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>),
            | -> (Factory, Option<Option<Factory>>) {
                let (v1, v2) = x;
                let v1 = Self::downcast(v1);
                let v2 = ::dyn_std::map::Map1::map(
                    v2,
                    |x: Option<Box<dyn Meta<T>>>| -> Option<Factory> {
                        ::dyn_std::map::Map1::map(
                            x,
                            |x: Box<dyn Meta<T>>| -> Factory { Self::downcast(x) },
                        )
                    },
                );
                (v1, v2)
            },
        );
        Factory::nested(v1)
    }
}
