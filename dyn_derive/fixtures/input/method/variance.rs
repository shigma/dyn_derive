#[dyn_trait]
trait Meta<T> {
    fn method_1(arg: &dyn Fn(Self) -> Self);
    fn method_2(arg: &mut dyn FnMut(Self) -> Self);
    fn method_3(arg: Box<dyn Fn(Self) -> Self>);
    fn method_4(arg: Box<dyn FnMut(Self) -> Self>);
}
