trait Meta<T>: ::dyn_std::any::Dyn {
    fn method_1(&self, arg: i32);
    fn method_2(&self, arg: Vec<T>);
    fn method_3(&self, arg1: i32, arg2: (Rc<T>, Result<(), T>));
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn method_1(arg: i32);
    fn method_2(arg: Vec<T>);
    fn method_3(arg1: i32, arg2: (Rc<T>, Result<(), T>));
}
#[automatically_derived]
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, a1: i32) {
        Factory::method_1(a1)
    }
    #[inline]
    fn method_2(&self, a1: Vec<T>) {
        Factory::method_2(a1)
    }
    #[inline]
    fn method_3(&self, a1: i32, a2: (Rc<T>, Result<(), T>)) {
        Factory::method_3(a1, a2)
    }
}
