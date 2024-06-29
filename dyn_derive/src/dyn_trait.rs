use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

pub fn main(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut cons: syn::ItemTrait = syn::parse2(input).expect("expect trait");
    let mut inst = cons.clone();
    let inst_ident = &inst.ident;
    let cons_ident = format_ident!("{}Static", inst_ident);
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
    cons.supertraits.push(syn::parse_quote! { 'static });
    let inst_params = inst.generics.params.into_iter().filter_map(|param| {
        match param {
            syn::GenericParam::Type(param) => Some(param),
            _ => None,
        }
    }).collect::<Vec<_>>();
    let where_clause = inst.generics.where_clause;
    inst.generics = Default::default();
    let mut ty_params = vec![];
    let mut cons_params = cons.generics.params.iter_mut().filter_map(|param| {
        let syn::GenericParam::Type(param) = param else {
            return None
        };
        let ident = &param.ident;
        ty_params.push(quote! { #ident });
        for bound in &mut param.bounds {
            match bound {
                syn::TypeParamBound::Trait(bound) => {
                    let op = bound.path.to_token_stream().to_string();
                    let name = format_ident!("{}Static", op);
                    bound.path = syn::parse_quote! { #name };
                },
                _ => {},
            }
        }
        Some(param.clone())
    }).collect::<Vec<_>>();
    let mut cons_items = vec![];
    for item in &mut inst.items {
        match item {
            syn::TraitItem::Fn(inst_fn) => {
                // inst_fn.default = None;
                let recv_arg = inst_fn.sig.receiver().map(|_| quote! { self });
                if recv_arg.is_none() {
                    inst_fn.sig.inputs.insert(0, syn::parse_quote! { &self });
                }
                let occurrence = match &mut inst_fn.sig.output {
                    syn::ReturnType::Type(_, ty) => {
                        Occurrence::substitute(ty.as_mut(), &|name| {
                            if name == "Self" {
                                return Some(syn::parse_quote! { Box<dyn #inst_ident> })
                            }
                            for param in &inst_params {
                                if param.ident == name {
                                    let bounds = &param.bounds;
                                    return Some(syn::parse_quote! { Box<dyn #bounds>})
                                }
                            }
                            None
                        })
                    },
                    _ => Occurrence::None,
                };
                let mut impl_fn = inst_fn.clone();
                impl_fn.attrs.push(syn::parse_quote! { #[inline] });
                let ident = &impl_fn.sig.ident;
                let args = impl_fn.sig.inputs.iter_mut().enumerate().flat_map(|(i, arg)| {
                    match arg {
                        syn::FnArg::Typed(arg) => {
                            let ident = format_ident!("arg{}", i);
                            arg.pat = syn::parse_quote! { #ident };
                            Some(quote! { #ident })
                        },
                        syn::FnArg::Receiver(_) => None,
                    }
                });
                let invocation = quote! { #ident(#(#args),*) };
                let body = occurrence.transform(match recv_arg {
                    Some(_) => quote! { self.0.#invocation },
                    None => quote! { <T as #cons_ident>::#invocation },
                });
                impl_fn.default = Some(syn::parse_quote! {{ #body }});
                cons_items.push(impl_fn);
            },
            _ => {},
        }
    }
    cons_params.push(match ty_params.len() {
        0 => syn::parse_quote! { T: #cons_ident },
        _ => syn::parse_quote! { T: #cons_ident<#(#ty_params),*> },
    });
    quote! {
        #inst
        #(#super_impls)*
        #cons
        impl<#(#cons_params),*> #inst_ident for ::dyn_std::inst::Instance<T, (#(#ty_params,)*)> #where_clause {
            #(#cons_items)*
        }
    }
}

#[derive(Debug, Clone)]
enum Occurrence {
    Exact,
    Args(Vec<Occurrence>, Vec<syn::Type>),
    Tuple(Vec<Occurrence>),
    None,
}

impl Occurrence {
    fn substitute(ty: &mut syn::Type, f: &impl Fn(String) -> Option<syn::Type>) -> Self {
        match ty {
            syn::Type::Path(tp) => {
                let name = tp.path.to_token_stream().to_string();
                let result = f(name);
                if let Some(repl) = result {
                    *ty = repl;
                    return Self::Exact
                }
                let syn::PathArguments::AngleBracketed(args) = &mut tp.path.segments.last_mut().unwrap().arguments else {
                    return Self::None
                };
                let mut nothing = true;
                let mut ts = args.args
                    .iter_mut()
                    .filter_map(|arg| {
                        match arg {
                            syn::GenericArgument::Type(ty) => Some(ty),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>();
                let os = ts.iter_mut().map(|ty| {
                    let o = Self::substitute(ty, f);
                    if !matches!(o, Self::None) {
                        nothing = false;
                    }
                    o
                }).collect::<Vec<_>>();
                if nothing {
                    return Self::None
                } else {
                    return Self::Args(os, ts.into_iter().map(|t| t.clone()).collect())
                }
            },
            syn::Type::Tuple(tt) => {
                let mut nothing = true;
                let mut ts = tt.elems.iter_mut().collect::<Vec<_>>();
                let os = ts.iter_mut().map(|ty| {
                    let o = Self::substitute(ty, f);
                    if !matches!(o, Self::None) {
                        nothing = false;
                    }
                    o
                }).collect::<Vec<_>>();
                if nothing {
                    return Self::None
                } else {
                    return Self::Tuple(os)
                }
            },
            _ => Self::None,
        }
    }

    fn transform(&self, body: TokenStream) -> TokenStream {
        match self {
            Occurrence::Exact => quote! { Box::new(::dyn_std::inst::Instance::new(#body)) },
            Occurrence::None => quote! { #body },
            Occurrence::Args(os, ts) => {
                let len = os.len();
                let ident = format_ident!("Map{}", len);
                let args = os.iter().map(|o| {
                    let body = o.transform(quote! { x });
                    quote! { |x| #body }
                });
                quote! { ::dyn_std::map::#ident::map::<#(#ts),*>(#body, #(#args),*) }
            },
            Occurrence::Tuple(os) => {
                let idents = (0..os.len()).map(|i| format_ident!("v{}", i + 1));
                let values = os.iter().enumerate().map(|(i, o)| {
                    let ident = format_ident!("v{}", i + 1);
                    o.transform(quote! { #ident })
                });
                quote! {
                    match #body {
                        (#(#idents),*) => (#(#values),*)
                    }
                }
            },
        }
    }
}
