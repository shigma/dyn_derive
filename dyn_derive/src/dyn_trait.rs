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
    inst.supertraits.push(syn::parse_quote! { ::dyn_std::any::Dyn });
    let inst_params = inst.generics.params.into_iter().filter_map(|param| {
        match param {
            syn::GenericParam::Type(mut param) => {
                for bound in &mut param.bounds {
                    match bound {
                        syn::TypeParamBound::Trait(bound) => {
                            let last = bound.path.segments.last_mut().unwrap();
                            let args = std::mem::replace(&mut last.arguments, Default::default());
                            match args {
                                syn::PathArguments::None => {
                                    return Some((param, vec![]))
                                },
                                syn::PathArguments::AngleBracketed(args) => {
                                    return Some((param, args.args.into_iter().map(|arg| {
                                        match arg {
                                            syn::GenericArgument::Type(ty) => ty,
                                            _ => unimplemented!(),
                                        }
                                    }).collect::<Vec<_>>()))
                                },
                                _ => unimplemented!(),
                            }
                        },
                        _ => {},
                    }
                }
                None
            },
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
                    let last = bound.path.segments.last_mut().unwrap();
                    last.ident = format_ident!("{}Static", last.ident);
                },
                _ => {},
            }
        }
        Some(param.clone())
    }).collect::<Vec<_>>();
    let cons_trait = match ty_params.len() {
        0 => quote! { #cons_ident },
        _ => quote! { #cons_ident<#(#ty_params),*> },
    };
    let mut cons_items = vec![];
    for item in &mut inst.items {
        match item {
            syn::TraitItem::Fn(inst_fn) => {
                // inst_fn.default = None;
                let recv_arg = inst_fn.sig.receiver().map(|_| quote! { self });
                if recv_arg.is_none() {
                    inst_fn.sig.inputs.insert(0, syn::parse_quote! { &self });
                }
                let f = |name| {
                    if name == "Self" {
                        return Some((
                            syn::parse_quote! { Box<dyn #inst_ident> },
                            syn::parse_quote! { Self },
                        ))
                    }
                    for (param, args) in &inst_params {
                        if param.ident == name {
                            let bounds = &param.bounds;
                            let ident = &param.ident;
                            return Some((
                                syn::parse_quote! { Box<dyn #bounds> },
                                syn::parse_quote! { ::dyn_std::Instance<#ident, (#(#args,)*)> },
                            ))
                        }
                    }
                    None
                };
                let stmts = inst_fn.sig.inputs.iter_mut().enumerate().filter_map(|(i, arg)| {
                    let syn::FnArg::Typed(arg) = arg else {
                        return None
                    };
                    let occurrence = Occurrence::substitute(&mut arg.ty, &f);
                    let ident = format_ident!("v{}", i);
                    if let Some(body) = occurrence.downcast_expr(quote! { #ident }) {
                        arg.pat = Box::new(syn::parse_quote! { #ident });
                        Some(quote! { let #ident = #body; })
                    } else {
                        None
                    }
                }).collect::<Vec<_>>();
                let output = match &mut inst_fn.sig.output {
                    syn::ReturnType::Type(_, ty) => {
                        Occurrence::substitute(ty.as_mut(), &f)
                    },
                    _ => Occurrence::None,
                };
                let mut impl_fn = inst_fn.clone();
                impl_fn.attrs.push(syn::parse_quote! { #[inline] });
                let ident = &impl_fn.sig.ident;
                let args = impl_fn.sig.inputs.iter_mut().flat_map(|arg| {
                    match arg {
                        syn::FnArg::Typed(arg) => {
                            let ident = &arg.pat;
                            Some(quote! { #ident })
                        },
                        syn::FnArg::Receiver(_) => None,
                    }
                });
                let invocation = quote! { #ident(#(#args),*) };
                let body = output.upcast_expr(match recv_arg {
                    Some(_) => quote! { self.0.#invocation },
                    None => quote! { <T as #cons_trait>::#invocation },
                });
                impl_fn.default = Some(syn::parse_quote! {{ #(#stmts)* #body }});
                cons_items.push(impl_fn);
            },
            _ => {},
        }
    }
    cons_params.push(syn::parse_quote! { T: #cons_trait });
    quote! {
        #inst
        #(#super_impls)*
        #cons
        impl<#(#cons_params),*> #inst_ident for ::dyn_std::Instance<T, (#(#ty_params,)*)> #where_clause {
            #(#cons_items)*
        }
    }
}

#[derive(Debug, Clone)]
enum Occurrence {
    Exact(syn::Type),
    Args(Vec<Occurrence>, Vec<syn::Type>),
    Tuple(Vec<Occurrence>),
    Ref(Box<Occurrence>, bool),
    None,
}

impl Occurrence {
    fn substitute(ty: &mut syn::Type, f: &impl Fn(String) -> Option<(syn::Type, syn::Type)>) -> Self {
        match ty {
            syn::Type::Path(tp) => {
                let name = tp.path.to_token_stream().to_string();
                let result = f(name);
                if let Some((repl, repl2)) = result {
                    *ty = repl;
                    return Self::Exact(repl2)
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
            syn::Type::Reference(tr) => {
                let o = Self::substitute(&mut tr.elem, f);
                if matches!(o, Self::None) {
                    return Self::None
                } else {
                    return Self::Ref(Box::new(o), tr.mutability.is_some())
                }
            },
            _ => Self::None,
        }
    }

    fn downcast_expr(&self, ident: TokenStream) -> Option<TokenStream> {
        match self {
            Occurrence::Exact(ty) => Some(quote! { #ident.as_any_box().downcast::<#ty>().unwrap().0 }),
            Occurrence::Ref(o, mutability) => match o.as_ref() {
                Occurrence::Exact(ty) => match mutability {
                    true => Some(quote! { &mut #ident.as_any_mut().downcast_mut::<#ty>().unwrap().0 }),
                    false => Some(quote! { &#ident.as_any().downcast_ref::<#ty>().unwrap().0 }),
                },
                Occurrence::None => None,
                _ => unimplemented!(),
            },
            Occurrence::None => None,
            _ => unimplemented!(),
        }
    }

    fn upcast_expr(&self, body: TokenStream) -> TokenStream {
        match self {
            Occurrence::Exact(_) => quote! { Box::new(::dyn_std::Instance::new(#body)) },
            Occurrence::None => quote! { #body },
            Occurrence::Args(os, ts) => {
                let len = os.len();
                let ident = format_ident!("Map{}", len);
                let args = os.iter().map(|o| {
                    let body = o.upcast_expr(quote! { x });
                    quote! { |x| #body }
                });
                quote! { ::dyn_std::cast::#ident::map::<#(#ts),*>(#body, #(#args),*) }
            },
            Occurrence::Tuple(os) => {
                let idents = (0..os.len()).map(|i| format_ident!("v{}", i + 1));
                let values = os.iter().enumerate().map(|(i, o)| {
                    let ident = format_ident!("v{}", i + 1);
                    o.upcast_expr(quote! { #ident })
                });
                quote! {
                    match #body {
                        (#(#idents),*) => (#(#values),*)
                    }
                }
            },
            Occurrence::Ref(_, _) => unreachable!(),
        }
    }
}
