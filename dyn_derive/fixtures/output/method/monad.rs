trait Meta<T: 'static>: Sized + 'static {
    fn option(arg: Option<Self>);
    fn result_1(arg: Result<Self, ()>);
    fn result_2(arg: Result<(), Self>);
    fn vec(arg: Vec<Self>);
    fn nested(arg: Vec<(Self, Option<Option<Self>>)>);
}
trait MetaInstance<T>: ::dyn_std::any::Dyn {}
trait MetaConstructor<T> {
    fn option(&self, arg: Option<Box<dyn MetaInstance<T>>>);
    fn result_1(&self, arg: Result<Box<dyn MetaInstance<T>>, ()>);
    fn result_2(&self, arg: Result<(), Box<dyn MetaInstance<T>>>);
    fn vec(&self, arg: Vec<Box<dyn MetaInstance<T>>>);
    fn nested(
        &self,
        arg: Vec<(Box<dyn MetaInstance<T>>, Option<Option<Box<dyn MetaInstance<T>>>>)>,
    );
}
#[automatically_derived]
impl<T: 'static, Factory: Meta<T>> MetaInstance<T> for ::dyn_std::Instance<Factory> {}
#[automatically_derived]
impl<T: 'static, Factory: Meta<T>> MetaConstructor<T>
for ::dyn_std::Constructor<Factory> {
    #[inline]
    fn option(&self, a1: Option<Box<dyn MetaInstance<T>>>) {
        let a1 = ::dyn_std::map::Map1::map(
            a1,
            |x: Box<dyn MetaInstance<T>>| -> Factory {
                ::dyn_std::Instance::<Factory>::downcast(x)
            },
        );
        Factory::option(a1)
    }
    #[inline]
    fn result_1(&self, a1: Result<Box<dyn MetaInstance<T>>, ()>) {
        let a1 = ::dyn_std::map::Map2::map(
            a1,
            |x: Box<dyn MetaInstance<T>>| -> Factory {
                ::dyn_std::Instance::<Factory>::downcast(x)
            },
            |x: ()| x,
        );
        Factory::result_1(a1)
    }
    #[inline]
    fn result_2(&self, a1: Result<(), Box<dyn MetaInstance<T>>>) {
        let a1 = ::dyn_std::map::Map2::map(
            a1,
            |x: ()| x,
            |x: Box<dyn MetaInstance<T>>| -> Factory {
                ::dyn_std::Instance::<Factory>::downcast(x)
            },
        );
        Factory::result_2(a1)
    }
    #[inline]
    fn vec(&self, a1: Vec<Box<dyn MetaInstance<T>>>) {
        let a1 = ::dyn_std::map::Map1::map(
            a1,
            |x: Box<dyn MetaInstance<T>>| -> Factory {
                ::dyn_std::Instance::<Factory>::downcast(x)
            },
        );
        Factory::vec(a1)
    }
    #[inline]
    fn nested(
        &self,
        a1: Vec<(Box<dyn MetaInstance<T>>, Option<Option<Box<dyn MetaInstance<T>>>>)>,
    ) {
        let a1 = ::dyn_std::map::Map1::map(
            a1,
            |
                (
                    b1,
                    b2,
                ): (Box<dyn MetaInstance<T>>, Option<Option<Box<dyn MetaInstance<T>>>>),
            | -> (Factory, Option<Option<Factory>>) {
                let b1 = ::dyn_std::Instance::<Factory>::downcast(b1);
                let b2 = ::dyn_std::map::Map1::map(
                    b2,
                    |x: Option<Box<dyn MetaInstance<T>>>| -> Option<Factory> {
                        ::dyn_std::map::Map1::map(
                            x,
                            |x: Box<dyn MetaInstance<T>>| -> Factory {
                                ::dyn_std::Instance::<Factory>::downcast(x)
                            },
                        )
                    },
                );
                (b1, b2)
            },
        );
        Factory::nested(a1)
    }
}
