trait Meta<T>: ::dyn_std::any::Dyn {
    fn method_1(&self, v2: &dyn Fn(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>);
    fn method_2(&self, mut v2: &mut dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>);
    fn method_3(&self, v2: Box<dyn Fn(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>);
    fn method_4(&self, mut v2: Box<dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>);
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn method_1(arg: &dyn Fn(Self) -> Self);
    fn method_2(arg: &mut dyn FnMut(Self) -> Self);
    fn method_3(arg: Box<dyn Fn(Self) -> Self>);
    fn method_4(arg: Box<dyn FnMut(Self) -> Self>);
}
#[automatically_derived]
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, v2: &dyn Fn(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>) {
        Factory::method_1(
            &|vv1| Self::downcast(v2(Box::new(::dyn_std::Instance::new(vv1)))),
        )
    }
    #[inline]
    fn method_2(&self, mut v2: &mut dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>) {
        Factory::method_2(
            &mut |vv1| Self::downcast(v2(Box::new(::dyn_std::Instance::new(vv1)))),
        )
    }
    #[inline]
    fn method_3(&self, v2: Box<dyn Fn(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>) {
        Factory::method_3(
            Box::new(move |vv1| Self::downcast(
                v2(Box::new(::dyn_std::Instance::new(vv1))),
            )),
        )
    }
    #[inline]
    fn method_4(&self, mut v2: Box<dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>>) {
        Factory::method_4(
            Box::new(move |vv1| Self::downcast(
                v2(Box::new(::dyn_std::Instance::new(vv1))),
            )),
        )
    }
}
