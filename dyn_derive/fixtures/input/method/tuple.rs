#[dyn_trait]
trait Meta<T> {
    fn tuple_1(arg1: ((Self, T), Self), arg2: T);
    fn tuple_2() -> (T, (Self, (T, Self)));
    fn tuple_3(arg: (T, Option<Self>)) -> Vec<(Self, T)>;
    fn tuple_4(arg: (T, Vec<T>)) -> (Option<T>, T);
}
