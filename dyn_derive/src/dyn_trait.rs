use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

pub fn main(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut cons: syn::ItemTrait = syn::parse2(input).expect("expect trait");
    let mut inst = cons.clone();
    let inst_ident = &inst.ident;
    let cons_ident = format_ident!("{}Constructor", inst_ident);
    let mut super_impls = vec![];
    cons.ident = cons_ident.clone();
    let mut is_sized = false;
    inst.supertraits = syn::punctuated::Punctuated::from_iter(cons.supertraits.iter_mut().flat_map(|param| {
        let syn::TypeParamBound::Trait(cons_bound) = param else {
            return Some(param.clone())
        };
        let mut inst_bound = cons_bound.clone();
        let op = inst_bound.path.to_token_stream().to_string();
        match op.as_str() {
            "Sized" => {
                is_sized = true;
                return None
            },
            "Clone" => {
                inst_bound.path = syn::parse_quote! { ::dyn_std::clone::Clone };
                super_impls.push(quote! {
                    impl Clone for Box<dyn #inst_ident> {
                        #[inline]
                        fn clone(&self) -> Self {
                            ::dyn_std::Fat::to_box(self, ::dyn_std::clone::Clone::dyn_clone)
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
                inst_bound.path = syn::parse_quote! { ::dyn_std::cmp::#name };
                super_impls.push(quote! {
                    impl core::cmp::#name for dyn #inst_ident {
                        #[inline]
                        fn #method(&self, other: &Self) -> #return_type {
                            self.#dyn_method(other.as_any())
                        }
                    }
                });
                #[cfg(feature = "extra-cmp-impl")]
                // Workaround Rust compiler bug:
                // https://github.com/rust-lang/rust/issues/31740#issuecomment-700950186
                super_impls.push(quote! {
                    impl core::cmp::#name<&Self> for Box<dyn #inst_ident> {
                        #[inline]
                        fn #method(&self, other: &&Self) -> #return_type {
                            self.#dyn_method(other.as_any())
                        }
                    }
                });
            },
            "Neg" | "Not" => {
                let name = format_ident!("{}", op);
                let method = format_ident!("{}", op.to_lowercase());
                let dyn_method = format_ident!("dyn_{}", method);
                inst_bound.path = syn::parse_quote! { ::dyn_std::ops::#name };
                cons_bound.path = syn::parse_quote! { #name<Output = Self> };
                super_impls.push(quote! {
                    impl std::ops::#name for Box<dyn #inst_ident> {
                        type Output = Self;
                        #[inline]
                        fn #method(self) -> Self {
                            ::dyn_std::Fat::into_box(self, |m| m.#dyn_method())
                        }
                    }
                });
            },
            "Add" | "Sub" | "Mul" | "Div" | "Rem" |
            "BitAnd" | "BitOr" | "BitXor" | "Shl" | "Shr" => {
                let name = format_ident!("{}", op);
                let method = format_ident!("{}", op.to_lowercase());
                let dyn_method = format_ident!("dyn_{}", method);
                inst_bound.path = syn::parse_quote! { ::dyn_std::ops::#name };
                cons_bound.path = syn::parse_quote! { #name<Output = Self> };
                super_impls.push(quote! {
                    impl std::ops::#name for Box<dyn #inst_ident> {
                        type Output = Self;
                        #[inline]
                        fn #method(self, other: Self) -> Self {
                            ::dyn_std::Fat::into_box(self, |m| m.#dyn_method(other.as_any_box()))
                        }
                    }
                });
            },
            "AddAssign" | "SubAssign" | "MulAssign" | "DivAssign" | "RemAssign" |
            "BitAndAssign" | "BitOrAssign" | "BitXorAssign" | "ShlAssign" | "ShrAssign" => {
                let name = format_ident!("{}", op);
                let method = format_ident!("{}_assign", op[0..op.len() - 6].to_lowercase());
                let dyn_method = format_ident!("dyn_{}_assign", method);
                inst_bound.path = syn::parse_quote! { ::dyn_std::ops::#name };
                super_impls.push(quote! {
                    impl std::ops::#name for Box<dyn #inst_ident> {
                        #[inline]
                        fn #method(&mut self, other: Self) {
                            self.#dyn_method(other.as_any_box())
                        }
                    }
                });
            },
            _ => {},
        }
        Some(syn::TypeParamBound::Trait(inst_bound))
    }));
    if !is_sized {
        cons.supertraits.push(syn::parse_quote! { Sized });
    }
    let self_repl: syn::Type = syn::parse_quote! { Box<dyn #inst_ident> };
    let mut cons_items = vec![];
    for item in &mut inst.items {
        match item {
            syn::TraitItem::Fn(inst_fn) => {
                // inst_fn.default = None;
                let recv_arg = inst_fn.sig.receiver().map(|_| quote! { self });
                if recv_arg.is_none() {
                    inst_fn.sig.inputs.insert(0, syn::parse_quote! { &self });
                }
                // let path_indices = vec![];
                let has_self_output = match &mut inst_fn.sig.output {
                    syn::ReturnType::Type(_, ty) => {
                        subst_type(ty.as_mut(), &self_repl)
                    },
                    _ => false,
                };
                let mut impl_fn = inst_fn.clone();
                impl_fn.attrs.push(syn::parse_quote! { #[inline] });
                let ident = &impl_fn.sig.ident;
                let args = impl_fn.sig.inputs.iter_mut().enumerate().flat_map(|(i, arg)| {
                    match arg {
                        syn::FnArg::Typed(arg) => {
                            let ident = format_ident!("arg{}", i);
                            arg.pat = syn::parse_quote! { #ident };
                            Some(syn::parse_quote! { #ident })
                        },
                        syn::FnArg::Receiver(_) => recv_arg.clone(),
                    }
                });
                let mut body = quote! {
                    <T as #cons_ident>::#ident(#(#args),*)
                };
                if has_self_output {
                    body = quote! { Box::new(#body) };
                }
                impl_fn.default = Some(syn::parse_quote! {{ #body }});
                cons_items.push(impl_fn);
            },
            _ => {},
        }
    }
    quote! {
        #inst
        #(#super_impls)*
        #cons
        impl<T: 'static + #cons_ident> #inst_ident for T {
            #(#cons_items)*
        }
    }
}

fn subst_type(ty: &mut syn::Type, repl: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(tp) => {
            let name = tp.path.to_token_stream().to_string();
            if name == "Self" {
                *ty = repl.clone();
                return true
            }
            // tp.path
        },
        _ => {},
    }
    false
}
