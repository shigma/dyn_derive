trait Meta<T: 'static>: Sized + 'static {
    fn method_1(self);
    fn method_2(&self);
    fn method_3(self: Box<Self>);
}
trait MetaInstance<T>: ::dyn_std::any::Dyn {
    fn method_1(self: Box<Self>);
    fn method_2(&self);
    fn method_3(self: Box<Self>);
}
trait MetaConstructor<T> {}
#[automatically_derived]
impl<T: 'static, Factory: Meta<T>> MetaInstance<T> for ::dyn_std::Instance<Factory> {
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
#[automatically_derived]
impl<T: 'static, Factory: Meta<T>> MetaConstructor<T>
for ::dyn_std::Constructor<Factory> {}
