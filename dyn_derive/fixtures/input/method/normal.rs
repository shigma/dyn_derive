#[dyn_trait]
trait Meta<T> {
    fn method_1(arg: i32);
    fn method_2(arg: Vec<T>);
    fn method_3(arg1: i32, arg2: (Rc<T>, Result<(), T>));
}
