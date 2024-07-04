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
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(&self, arg: i32) {
        <Factory as MetaFactory<T>>::method_1(arg)
    }
    #[inline]
    fn method_2(&self, arg: Vec<T>) {
        <Factory as MetaFactory<T>>::method_2(arg)
    }
    #[inline]
    fn method_3(&self, arg1: i32, arg2: (Rc<T>, Result<(), T>)) {
        <Factory as MetaFactory<T>>::method_3(arg1, arg2)
    }
}
