trait Meta<T>: ::dyn_std::any::Dyn {
    fn method_1(&self, arg: &dyn Fn(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>);
    fn method_2(&self, arg: &mut dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>);
    fn method_3(&self, arg: Box<dyn Fn(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>);
    fn method_4(&self, arg: Box<dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>);
    fn method_5(
        &self,
        arg: Box<
            dyn FnMut(
                Box<dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>,
            ) -> Box<dyn Meta<T>>,
        >,
    );
    fn method_6(
        &self,
        arg: Box<
            dyn FnOnce(
                Box<dyn Meta<T>>,
            ) -> Box<dyn FnOnce(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>,
        >,
    );
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn method_1(arg: &dyn Fn(Self) -> Self);
    fn method_2(arg: &mut dyn FnMut(Self) -> Self);
    fn method_3(arg: Box<dyn Fn(Self) -> Self>);
    fn method_4(arg: Box<dyn FnMut(Self) -> Self>);
    fn method_5(arg: Box<dyn FnMut(Box<dyn FnMut(Self) -> Self>) -> Self>);
    fn method_6(arg: Box<dyn FnOnce(Self) -> Box<dyn FnOnce(Self) -> Self>>);
}
#[automatically_derived]
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, v1: &dyn Fn(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>) {
        Factory::method_1(
            &|v1_1| Self::downcast(v1(Box::new(::dyn_std::Instance::new(v1_1)))),
        )
    }
    #[inline]
    fn method_2(&self, v1: &mut dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>) {
        Factory::method_2(
            &mut |v1_1| Self::downcast(v1(Box::new(::dyn_std::Instance::new(v1_1)))),
        )
    }
    #[inline]
    fn method_3(&self, v1: Box<dyn Fn(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>) {
        Factory::method_3(
            Box::new(move |v1_1| Self::downcast(
                v1(Box::new(::dyn_std::Instance::new(v1_1))),
            )),
        )
    }
    #[inline]
    fn method_4(&self, mut v1: Box<dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>) {
        Factory::method_4(
            Box::new(move |v1_1| Self::downcast(
                v1(Box::new(::dyn_std::Instance::new(v1_1))),
            )),
        )
    }
    #[inline]
    fn method_5(
        &self,
        mut v1: Box<
            dyn FnMut(
                Box<dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>,
            ) -> Box<dyn Meta<T>>,
        >,
    ) {
        Factory::method_5(
            Box::new(move |mut v1_1| Self::downcast(
                v1(
                    Box::new(move |v2_1| Box::new(
                        ::dyn_std::Instance::new(v1_1(Self::downcast(v2_1))),
                    )),
                ),
            )),
        )
    }
    #[inline]
    fn method_6(
        &self,
        v1: Box<
            dyn FnOnce(
                Box<dyn Meta<T>>,
            ) -> Box<dyn FnOnce(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>,
        >,
    ) {
        Factory::method_6(
            Box::new(move |v1_1| Box::new(move |v2_1| Self::downcast(
                v1(
                    Box::new(::dyn_std::Instance::new(v1_1)),
                )(Box::new(::dyn_std::Instance::new(v2_1))),
            ))),
        )
    }
}
