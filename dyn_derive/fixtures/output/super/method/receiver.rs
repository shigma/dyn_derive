trait Meta<T>: ::dyn_std::any::Dyn {
    fn recv_1(self: Box<Self>);
    fn recv_2(&self);
    fn recv_3(self: Box<Self>);
}
trait MetaFactory<T: 'static>: Sized + 'static {
    fn recv_1(self);
    fn recv_2(&self);
    fn recv_3(self: Box<Self>);
}
impl<T: 'static, Factory: MetaFactory<T>> Meta<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn recv_1(self: Box<Self>) {
        self.0.recv_1()
    }
    #[inline]
    fn recv_2(&self) {
        self.0.recv_2()
    }
    #[inline]
    fn recv_3(self: Box<Self>) {
        self.0.recv_3()
    }
}
