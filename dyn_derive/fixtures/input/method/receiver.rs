#[dyn_trait]
trait Meta<T> {
    fn method_1(self);
    fn method_2(&self);
    fn method_3(self: Box<Self>);
}
