use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

pub fn main(input: TokenStream) -> TokenStream {
    let mut item: syn::ItemTrait = syn::parse2(input).expect("expect trait");
    let ident = &item.ident;
    let mut impls = vec![];
    for param in &mut item.supertraits {
        let syn::TypeParamBound::Trait(bound) = param else {
            continue;
        };
        let op = bound.path.to_token_stream().to_string();
        match op.as_str() {
            "Clone" => {
                bound.path = syn::parse_quote! { dynex_core::DynClone };
                impls.push(quote! {
                    impl Clone for Box<dyn #ident> {
                        fn clone(&self) -> Self {
                            dynex_core::ptr::convert_to_box(self, dynex_core::DynClone::dyn_clone)
                        }
                    }
                });
            },
            "PartialEq" | "PartialOrd" => {
                let name = format_ident!("{}", op);
                let (method, dyn_method, return_type) = match op.as_str() {
                    "PartialEq" => (quote!(eq), quote!(dyn_eq), quote!(bool)),
                    "PartialOrd" => (quote!(partial_cmp), quote!(dyn_partial_cmp), quote!(Option<core::cmp::Ordering>)),
                    _ => unreachable!(),
                };
                bound.path = syn::parse_quote! { dynex_core::cmp::#name };
                impls.push(quote! {
                    impl core::cmp::#name for dyn #ident {
                        fn #method(&self, other: &Self) -> #return_type {
                            self.#dyn_method(other.as_any())
                        }
                    }
                });
            },
            "Neg" | "Not" => {
                let name = format_ident!("{}", op);
                let method = format_ident!("{}", op.to_lowercase());
                let dyn_method = format_ident!("dyn_{}", method);
                bound.path = syn::parse_quote! { dynex_core::ops::#name };
                impls.push(quote! {
                    impl std::ops::#name for Box<dyn #ident> {
                        type Output = Self;
                        fn #method(self) -> Self {
                            dynex_core::ptr::convert_into_box(self, |m| m.#dyn_method())
                        }
                    }
                });
            },
            "Add" | "Sub" | "Mul" | "Div" | "Rem" |
            "BitAnd" | "BitOr" | "BitXor" | "Shl" | "Shr" => {
                let name = format_ident!("{}", op);
                let method = format_ident!("{}", op.to_lowercase());
                let dyn_method = format_ident!("dyn_{}", method);
                bound.path = syn::parse_quote! { dynex_core::ops::#name };
                impls.push(quote! {
                    impl std::ops::#name for Box<dyn #ident> {
                        type Output = Self;
                        fn #method(self, other: Self) -> Self {
                            dynex_core::ptr::convert_into_box(self, |m| m.#dyn_method(other.as_any_box()))
                        }
                    }
                });
            },
            "AddAssign" | "SubAssign" | "MulAssign" | "DivAssign" | "RemAssign" |
            "BitAndAssign" | "BitOrAssign" | "BitXorAssign" | "ShlAssign" | "ShrAssign" => {
                let name = format_ident!("{}", op);
                let method = format_ident!("{}_assign", op[0..op.len() - 6].to_lowercase());
                let dyn_method = format_ident!("dyn_{}_assign", method);
                bound.path = syn::parse_quote! { dynex_core::ops::#name };
                impls.push(quote! {
                    impl std::ops::#name for Box<dyn #ident> {
                        fn #method(&mut self, other: Self) {
                            self.#dyn_method(other.as_any_box())
                        }
                    }
                });
            },
            _ => {},
        }
    }
    quote! {
        #item
        #(#impls)*
    }
}
