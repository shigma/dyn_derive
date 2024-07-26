trait MetaInstance<T>: ::dyn_std::any::Dyn {
    fn method_1(
        &self,
        arg: &dyn Fn(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>,
    );
    fn method_2(
        &self,
        arg: &mut dyn FnMut(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>,
    );
    fn method_3(
        &self,
        arg: Box<dyn Fn(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
    );
    fn method_4(
        &self,
        arg: Box<dyn FnMut(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
    );
    fn method_5(
        &self,
        arg: Box<
            dyn FnMut(
                Box<dyn FnMut(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
            ) -> Box<dyn MetaInstance<T>>,
        >,
    );
    fn method_6(
        &self,
        arg: Box<
            dyn FnOnce(
                Box<dyn MetaInstance<T>>,
            ) -> Box<dyn FnOnce(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
        >,
    );
    fn method_7(
        &self,
        arg: Box<dyn FnOnce(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
    ) -> Box<dyn FnOnce(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>;
}
trait Meta<T: 'static>: Sized + 'static {
    fn method_1(arg: &dyn Fn(Self) -> Self);
    fn method_2(arg: &mut dyn FnMut(Self) -> Self);
    fn method_3(arg: Box<dyn Fn(Self) -> Self>);
    fn method_4(arg: Box<dyn FnMut(Self) -> Self>);
    fn method_5(arg: Box<dyn FnMut(Box<dyn FnMut(Self) -> Self>) -> Self>);
    fn method_6(arg: Box<dyn FnOnce(Self) -> Box<dyn FnOnce(Self) -> Self>>);
    fn method_7(arg: Box<dyn FnOnce(Self) -> Self>) -> Box<dyn FnOnce(Self) -> Self>;
}
#[automatically_derived]
impl<T: 'static, Factory: Meta<T>> MetaInstance<T>
for ::dyn_std::Instance<Factory, (T,)> {
    #[inline]
    fn method_1(
        &self,
        a1: &dyn Fn(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>,
    ) {
        let a1 = &|b1| {
            let b1 = Box::new(::dyn_std::Instance::new(b1));
            Self::downcast(a1(b1))
        };
        Factory::method_1(a1)
    }
    #[inline]
    fn method_2(
        &self,
        a1: &mut dyn FnMut(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>,
    ) {
        let a1 = &mut |b1| {
            let b1 = Box::new(::dyn_std::Instance::new(b1));
            Self::downcast(a1(b1))
        };
        Factory::method_2(a1)
    }
    #[inline]
    fn method_3(
        &self,
        a1: Box<dyn Fn(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
    ) {
        let a1 = Box::new(move |b1| {
            let b1 = Box::new(::dyn_std::Instance::new(b1));
            Self::downcast(a1(b1))
        });
        Factory::method_3(a1)
    }
    #[inline]
    fn method_4(
        &self,
        a1: Box<dyn FnMut(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
    ) {
        let mut a1 = Box::new(move |b1| {
            let b1 = Box::new(::dyn_std::Instance::new(b1));
            Self::downcast(a1(b1))
        });
        Factory::method_4(a1)
    }
    #[inline]
    fn method_5(
        &self,
        a1: Box<
            dyn FnMut(
                Box<dyn FnMut(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
            ) -> Box<dyn MetaInstance<T>>,
        >,
    ) {
        let mut a1 = Box::new(move |b1| {
            let mut b1 = Box::new(move |c1| {
                let c1 = Self::downcast(c1);
                Box::new(::dyn_std::Instance::new(b1(c1)))
            });
            Self::downcast(a1(b1))
        });
        Factory::method_5(a1)
    }
    #[inline]
    fn method_6(
        &self,
        a1: Box<
            dyn FnOnce(
                Box<dyn MetaInstance<T>>,
            ) -> Box<dyn FnOnce(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
        >,
    ) {
        let a1 = Box::new(move |b1| {
            let b1 = Box::new(::dyn_std::Instance::new(b1));
            Box::new(move |c1| {
                let c1 = Box::new(::dyn_std::Instance::new(c1));
                Self::downcast(a1(b1)(c1))
            })
        });
        Factory::method_6(a1)
    }
    #[inline]
    fn method_7(
        &self,
        a1: Box<dyn FnOnce(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>>,
    ) -> Box<dyn FnOnce(Box<dyn MetaInstance<T>>) -> Box<dyn MetaInstance<T>>> {
        let a1 = Box::new(move |b1| {
            let b1 = Box::new(::dyn_std::Instance::new(b1));
            Self::downcast(a1(b1))
        });
        Box::new(move |b1| {
            let b1 = Self::downcast(b1);
            Box::new(::dyn_std::Instance::new(Factory::method_7(a1)(b1)))
        })
    }
}
