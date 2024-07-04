#[dyn_trait]
trait Meta<T> {
    fn recv_1(self);
    fn recv_2(&self);
    fn recv_3(self: Box<Self>);
}
