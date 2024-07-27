trait Meta<T: 'static>: Sized + 'static {
    fn tuple_1(arg1: ((Self, T), Self), arg2: T);
    fn tuple_2() -> (T, (Self, (T, Self)));
    fn tuple_3(arg: (T, Option<Self>)) -> Vec<(Self, T)>;
    fn tuple_4(arg: (T, Vec<T>)) -> (Option<T>, T);
}
trait MetaInstance<T>: ::dyn_std::any::Dyn {}
trait MetaConstructor<T> {
    fn tuple_1(
        &self,
        arg1: ((Box<dyn MetaInstance<T>>, T), Box<dyn MetaInstance<T>>),
        arg2: T,
    );
    fn tuple_2(&self) -> (T, (Box<dyn MetaInstance<T>>, (T, Box<dyn MetaInstance<T>>)));
    fn tuple_3(
        &self,
        arg: (T, Option<Box<dyn MetaInstance<T>>>),
    ) -> Vec<(Box<dyn MetaInstance<T>>, T)>;
    fn tuple_4(&self, arg: (T, Vec<T>)) -> (Option<T>, T);
}
#[automatically_derived]
impl<T: 'static, Factory: Meta<T>> MetaInstance<T> for ::dyn_std::Instance<Factory> {}
#[automatically_derived]
impl<T: 'static, Factory: Meta<T>> MetaConstructor<T>
for ::dyn_std::Constructor<Factory> {
    #[inline]
    fn tuple_1(
        &self,
        ((a1, a2), a3): ((Box<dyn MetaInstance<T>>, T), Box<dyn MetaInstance<T>>),
        a4: T,
    ) {
        let a1 = ::dyn_std::Instance::<Factory>::downcast(a1);
        let a3 = ::dyn_std::Instance::<Factory>::downcast(a3);
        Factory::tuple_1(((a1, a2), a3), a4)
    }
    #[inline]
    fn tuple_2(&self) -> (T, (Box<dyn MetaInstance<T>>, (T, Box<dyn MetaInstance<T>>))) {
        let (a1, (a2, (a3, a4))) = Factory::tuple_2();
        let a2 = Box::new(::dyn_std::Instance::new(a2));
        let a4 = Box::new(::dyn_std::Instance::new(a4));
        (a1, (a2, (a3, a4)))
    }
    #[inline]
    fn tuple_3(
        &self,
        (a1, a2): (T, Option<Box<dyn MetaInstance<T>>>),
    ) -> Vec<(Box<dyn MetaInstance<T>>, T)> {
        let a2 = ::dyn_std::map::Map1::map(
            a2,
            |x: Box<dyn MetaInstance<T>>| -> Factory {
                ::dyn_std::Instance::<Factory>::downcast(x)
            },
        );
        ::dyn_std::map::Map1::map(
            Factory::tuple_3((a1, a2)),
            |(b1, b2): (Factory, T)| -> (Box<dyn MetaInstance<T>>, T) {
                let b1 = Box::new(::dyn_std::Instance::new(b1));
                (b1, b2)
            },
        )
    }
    #[inline]
    fn tuple_4(&self, a1: (T, Vec<T>)) -> (Option<T>, T) {
        Factory::tuple_4(a1)
    }
}
