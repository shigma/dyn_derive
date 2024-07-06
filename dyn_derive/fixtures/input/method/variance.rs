#[dyn_trait]
trait Meta<T> {
    fn method_1(arg: &mut dyn FnMut(Self) -> Self);
}
