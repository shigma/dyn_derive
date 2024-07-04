trait Meta<T>: ::dyn_std::any::Dyn {
    fn option(&self, v1: Option<Box<dyn Meta<T>>>);
    fn result_1(&self, v1: Result<Box<dyn Meta<T>>, ()>);
    fn result_2(&self, v1: Result<(), Box<dyn Meta<T>>>);
    fn vec(&self, v1: Vec<Box<dyn Meta<T>>>);
    fn nested(&self, v1: Vec<(Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>)>);
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn option(arg: Option<Self>);
    fn result_1(arg: Result<Self, ()>);
    fn result_2(arg: Result<(), Self>);
    fn vec(arg: Vec<Self>);
    fn nested(arg: Vec<(Self, Option<Option<Self>>)>);
}
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn option(&self, v1: Option<Box<dyn Meta<T>>>) {
        let v1 = ::dyn_std::map::Map1::map::<
            Box<dyn Meta<T>>,
        >(v1, |x| Self::downcast(x));
        <Factory as MetaFactory<T>>::option(v1)
    }
    #[inline]
    fn result_1(&self, v1: Result<Box<dyn Meta<T>>, ()>) {
        let v1 = ::dyn_std::map::Map2::map::<
            Box<dyn Meta<T>>,
            (),
        >(v1, |x| Self::downcast(x), |x| x);
        <Factory as MetaFactory<T>>::result_1(v1)
    }
    #[inline]
    fn result_2(&self, v1: Result<(), Box<dyn Meta<T>>>) {
        let v1 = ::dyn_std::map::Map2::map::<
            (),
            Box<dyn Meta<T>>,
        >(v1, |x| x, |x| Self::downcast(x));
        <Factory as MetaFactory<T>>::result_2(v1)
    }
    #[inline]
    fn vec(&self, v1: Vec<Box<dyn Meta<T>>>) {
        let v1 = ::dyn_std::map::Map1::map::<
            Box<dyn Meta<T>>,
        >(v1, |x| Self::downcast(x));
        <Factory as MetaFactory<T>>::vec(v1)
    }
    #[inline]
    fn nested(&self, v1: Vec<(Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>)>) {
        let v1 = ::dyn_std::map::Map1::map::<
            (Box<dyn Meta<T>>, Option<Option<Box<dyn Meta<T>>>>),
        >(
            v1,
            |x| match x {
                (v1, v2) => {
                    (
                        Self::downcast(v1),
                        ::dyn_std::map::Map1::map::<
                            Option<Box<dyn Meta<T>>>,
                        >(
                            v2,
                            |x| ::dyn_std::map::Map1::map::<
                                Box<dyn Meta<T>>,
                            >(x, |x| Self::downcast(x)),
                        ),
                    )
                }
            },
        );
        <Factory as MetaFactory<T>>::nested(v1)
    }
}
