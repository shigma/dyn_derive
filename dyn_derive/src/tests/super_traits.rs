use quote::quote;

use super::utils::assert_transform;

#[test]
fn debug() {
    assert_transform(quote! {
        #[dyn_trait]
        trait Meta: Debug {}
    }, quote! {
        trait Meta: Debug + ::dyn_std::any::Dyn {}

        trait MetaFactory: Debug + Sized + 'static {}

        impl<Factory: MetaFactory> Meta for ::dyn_std::Instance<Factory, ()> {}
    });
}

#[test]
fn add() {
    assert_transform(quote! {
        #[dyn_trait]
        trait Meta: Add {}
    }, quote! {
        trait Meta: ::dyn_std::ops::Add + ::dyn_std::any::Dyn {}

        impl std::ops::Add for Box<dyn Meta> {
            type Output = Self;
            #[inline]
            fn add(self, other: Self) -> Self {
                ::dyn_std::Fat::into_box(self, |m| m.dyn_add(other.as_any_box()))
            }
        }

        trait MetaFactory: Add<Output = Self> + Sized + 'static {}

        impl<Factory: MetaFactory> Meta for ::dyn_std::Instance<Factory, ()> {}
    });
}

#[test]
fn clone() {
    assert_transform(quote! {
        #[dyn_trait]
        trait Meta: Clone {}
    }, quote! {
        trait Meta: ::dyn_std::clone::Clone + ::dyn_std::any::Dyn {}

        impl Clone for Box<dyn Meta> {
            #[inline]
            fn clone(&self) -> Self {
                ::dyn_std::Fat::to_box(self, ::dyn_std::clone::Clone::dyn_clone)
            }
        }

        trait MetaFactory: Clone + Sized + 'static {}

        impl<Factory: MetaFactory> Meta for ::dyn_std::Instance<Factory, ()> {}
    });
}

#[test]
fn partial_eq() {
    assert_transform(quote! {
        #[dyn_trait]
        trait Meta: PartialEq {}
    }, quote! {
        trait Meta: ::dyn_std::cmp::PartialEq + ::dyn_std::any::Dyn {}

        impl std::cmp::PartialEq for dyn Meta {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.dyn_eq(other.as_any())
            }
        }

        impl std::cmp::PartialEq<&Self> for Box<dyn Meta> {
            #[inline]
            fn eq(&self, other: &&Self) -> bool {
                self.dyn_eq(other.as_any())
            }
        }

        trait MetaFactory: PartialEq + Sized + 'static {}

        impl<Factory: MetaFactory> Meta for ::dyn_std::Instance<Factory, ()> {}
    });
}
