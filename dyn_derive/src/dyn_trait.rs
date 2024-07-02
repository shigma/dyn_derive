use std::collections::HashMap;
use std::mem::replace;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

fn supertraits(fact: &mut syn::ItemTrait, inst: &mut syn::ItemTrait) -> TokenStream {
    let mut has_sized = false;
    let mut output = quote! {};
    let inst_ident = &inst.ident;
    inst.supertraits = syn::punctuated::Punctuated::from_iter(fact.supertraits.iter_mut().flat_map(|param| {
        let syn::TypeParamBound::Trait(fact_bound) = param else {
            return Some(param.clone())
        };
        let mut inst_bound = fact_bound.clone();
        let op = inst_bound.path.to_token_stream().to_string();
        match op.as_str() {
            "Sized" => {
                has_sized = true;
                return None
            },
            "Clone" => {
                inst_bound.path = syn::parse_quote! { ::dyn_std::clone::Clone };
                output.extend(quote! {
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
                output.extend(quote! {
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
                output.extend(quote! {
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
                fact_bound.path = syn::parse_quote! { #name<Output = Self> };
                output.extend(quote! {
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
                fact_bound.path = syn::parse_quote! { #name<Output = Self> };
                output.extend(quote! {
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
                output.extend(quote! {
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
    if !has_sized {
        fact.supertraits.push(syn::parse_quote! { Sized });
    }
    fact.supertraits.push(syn::parse_quote! { 'static });
    inst.supertraits.push(syn::parse_quote! { ::dyn_std::any::Dyn });
    output
}

struct Generic {
    param: syn::TypeParam,
    args: Vec<syn::Type>,
}

fn collect_generics(inst: &mut syn::ItemTrait) -> HashMap<String, Generic> {
    let mut data = HashMap::new();
    let params = replace(&mut inst.generics.params, Default::default());
    for param in params {
        match param {
            syn::GenericParam::Type(mut param) => {
                // todo: multiple bounds
                for bound in &mut param.bounds {
                    match bound {
                        syn::TypeParamBound::Trait(bound) => {
                            let last = bound.path.segments.last_mut().unwrap();
                            let args = std::mem::replace(&mut last.arguments, Default::default());
                            match args {
                                syn::PathArguments::None => {
                                    data.insert(param.ident.to_string(), Generic {
                                        param,
                                        args: vec![],
                                    });
                                    break;
                                },
                                syn::PathArguments::AngleBracketed(args) => {
                                    data.insert(param.ident.to_string(), Generic {
                                        param,
                                        args: args.args.into_iter().map(|arg| {
                                            match arg {
                                                syn::GenericArgument::Type(ty) => ty,
                                                _ => unimplemented!(),
                                            }
                                        }).collect(),
                                    });
                                    break;
                                },
                                syn::PathArguments::Parenthesized(_) => unimplemented!("parenthesized bounds in trait generics"),
                            }
                        },
                        _ => {},
                    }
                }
            },
            syn::GenericParam::Const(_) => unimplemented!("const in trait generics"),
            syn::GenericParam::Lifetime(_) => unimplemented!("lifetime in trait generics"),
        }
    }
    data
}

fn match_generics(name: String, inst_ident: &syn::Ident, generics: &HashMap<String, Generic>) -> Option<(syn::Type, syn::Type)> {
    if name == "Self" {
        return Some((
            syn::parse_quote! { Box<dyn #inst_ident> },
            syn::parse_quote! { Self },
        ))
    }
    let Some(g) = generics.get(&name) else {
        return None
    };
    let ident = &g.param.ident;
    let bounds = &g.param.bounds;
    let args = &g.args;
    return Some((
        syn::parse_quote! { Box<dyn #bounds> },
        syn::parse_quote! { ::dyn_std::Instance<#ident, (#(#args,)*)> },
    ))
}

pub fn transform(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut fact: syn::ItemTrait = syn::parse2(input).expect("expect trait");
    let mut inst = fact.clone();
    let inst_ident = inst.ident.clone();
    let super_impls = supertraits(&mut fact, &mut inst);
    let fact_ident = format_ident!("{}Factory", inst_ident);
    let inst_generics = collect_generics(&mut inst);
    let mut fact_generic_params = vec![];
    let mut fact_generics = fact.generics.params.iter_mut().filter_map(|param| {
        let syn::GenericParam::Type(param) = param else {
            unimplemented!("const or lifetime in trait generics")
        };
        let ident = &param.ident;
        fact_generic_params.push(quote! { #ident });
        for bound in &mut param.bounds {
            match bound {
                syn::TypeParamBound::Trait(bound) => {
                    let last = bound.path.segments.last_mut().unwrap();
                    last.ident = format_ident!("{}Factory", last.ident);
                },
                _ => {},
            }
        }
        Some(param.clone())
    }).collect::<Vec<_>>();
    let fact_trait = match fact_generic_params.len() {
        0 => quote! { #fact_ident },
        _ => quote! { #fact_ident<#(#fact_generic_params),*> },
    };
    fact.ident = fact_ident;
    let mut fact_items = vec![];
    for item in &mut inst.items {
        match item {
            syn::TraitItem::Fn(inst_fn) => {
                // inst_fn.default = None;
                let recv_arg = inst_fn.sig.receiver().map(|_| quote! { self });
                if recv_arg.is_none() {
                    inst_fn.sig.inputs.insert(0, syn::parse_quote! { &self });
                }
                let stmts = inst_fn.sig.inputs.iter_mut().enumerate().filter_map(|(i, arg)| {
                    let syn::FnArg::Typed(arg) = arg else {
                        return None
                    };
                    let occurrence = Occurrence::substitute(&mut arg.ty, &|name| {
                        match_generics(name, &inst_ident, &inst_generics)
                    });
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
                        Occurrence::substitute(ty.as_mut(), &|name| {
                            match_generics(name, &inst_ident, &inst_generics)
                        })
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
                    None => quote! { <Factory as #fact_trait>::#invocation },
                });
                impl_fn.default = Some(syn::parse_quote! {{ #(#stmts)* #body }});
                fact_items.push(impl_fn);
            },
            _ => {},
        }
    }
    fact_generics.push(syn::parse_quote! { Factory: #fact_trait });
    let fact = &fact;
    let inst = &inst;
    let where_clause = &inst.generics.where_clause;
    quote! {
        #inst
        #super_impls
        #fact
        impl<#(#fact_generics),*> #inst_ident for ::dyn_std::Instance<Factory, (#(#fact_generic_params,)*)> #where_clause {
            #(#fact_items)*
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
                quote! { ::dyn_std::map::#ident::map::<#(#ts),*>(#body, #(#args),*) }
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
