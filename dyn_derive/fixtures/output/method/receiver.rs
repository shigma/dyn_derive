trait Meta<T>: ::dyn_std::any::Dyn {
    fn method_1(self: Box<Self>);
    fn method_2(&self);
    fn method_3(self: Box<Self>);
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn method_1(self);
    fn method_2(&self);
    fn method_3(self: Box<Self>);
}
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(self: Box<Self>) {
        self.0.method_1()
    }
    #[inline]
    fn method_2(&self) {
        self.0.method_2()
    }
    #[inline]
    fn method_3(self: Box<Self>) {
        self.0.method_3()
    }
}
