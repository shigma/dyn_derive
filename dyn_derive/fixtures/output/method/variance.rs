trait Meta<T>: ::dyn_std::any::Dyn {
    fn method_1(&self, v1: &mut dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>);
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn method_1(arg: &mut dyn FnMut(Self) -> Self);
}
#[automatically_derived]
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, v1: &mut dyn FnMut(Box<dyn Meta<T>>) -> Box<dyn Meta<T>>) {
        let v1 = &mut |vv1| {
            let vv1 = Box::new(::dyn_std::Instance::new(vv1));
            Self::downcast(v1(vv1))
        };
        Factory::method_1(v1)
    }
}
