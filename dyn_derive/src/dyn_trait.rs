use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::{generics::GenericsData, subst_self::subst_self};

fn supertraits(fact: &mut syn::ItemTrait, inst: &mut syn::ItemTrait) -> TokenStream {
    let mut has_sized = false;
    let inst_ident = &inst.ident;
    let (impl_generics, type_generics, where_clause) = inst.generics.split_for_impl();
    let mut output = quote! {};
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
                    #[automatically_derived]
                    impl #impl_generics Clone for Box<dyn #inst_ident #type_generics> #where_clause {
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
                    "PartialOrd" => (quote!(partial_cmp), quote!(dyn_partial_cmp), quote!(Option<std::cmp::Ordering>)),
                    _ => unreachable!(),
                };
                inst_bound.path = syn::parse_quote! { ::dyn_std::cmp::#name };
                output.extend(quote! {
                    #[automatically_derived]
                    impl #impl_generics std::cmp::#name for dyn #inst_ident #type_generics #where_clause {
                        #[inline]
                        fn #method(&self, other: &Self) -> #return_type {
                            self.#dyn_method(other.as_any())
                        }
                    }
                });
                // Workaround Rust compiler bug:
                // https://github.com/rust-lang/rust/issues/31740#issuecomment-700950186
                output.extend(quote! {
                    #[automatically_derived]
                    impl #impl_generics std::cmp::#name<&Self> for Box<dyn #inst_ident #type_generics> #where_clause {
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
                    #[automatically_derived]
                    impl #impl_generics std::ops::#name for Box<dyn #inst_ident #type_generics> #where_clause {
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
                    #[automatically_derived]
                    impl #impl_generics std::ops::#name for Box<dyn #inst_ident #type_generics> #where_clause {
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
                    #[automatically_derived]
                    impl #impl_generics std::ops::#name for Box<dyn #inst_ident #type_generics> #where_clause {
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

fn make_mut_prefix(mutability: bool) -> TokenStream {
    if mutability {
        quote! { mut }
    } else {
        quote! {}
    }
}

pub fn get_full_name(item: &syn::ItemTrait) -> (TokenStream, TokenStream) {
    let ident = &item.ident;
    let mut generic_params = vec![];
    let mut instance_params = vec![];
    for param in &item.generics.params {
        match param {
            syn::GenericParam::Type(param) => {
                let ident = &param.ident;
                generic_params.push(quote! { #ident });
                instance_params.push(quote! { #ident });
            },
            syn::GenericParam::Lifetime(param) => {
                let lifetime = &param.lifetime;
                generic_params.push(quote! { #lifetime });
            },
            syn::GenericParam::Const(_) => {
                unimplemented!("const generics in traits")
            },
        }
    }
    (
        match generic_params.len() {
            0 => quote! { #ident },
            _ => quote! { #ident<#(#generic_params),*> },
        },
        match instance_params.len() {
            0 => quote! { () },
            _ => quote! { (#(#instance_params),*,) },
        },
    )
}

pub fn transform(_attr: TokenStream, mut fact: syn::ItemTrait) -> TokenStream {
    let mut inst = fact.clone();
    let inst_ident = inst.ident.clone();
    let fact_ident = format_ident!("{}Factory", inst_ident);
    let generics = GenericsData::from(&mut inst);
    let (inst_trait, _) = get_full_name(&inst);
    let super_impls = supertraits(&mut fact, &mut inst);
    fact.generics.params.iter_mut().for_each(|param| {
        let syn::GenericParam::Type(param) = param else {
            return;
        };
        let index = param.attrs.iter().position(|attr| {
            attr.meta.path().is_ident("dynamic")
        });
        if let Some(index) = index {
            param.attrs.remove(index);
        } else {
            param.bounds.push(syn::parse_quote! { 'static });
            return;
        }
        for bound in &mut param.bounds {
            match bound {
                syn::TypeParamBound::Trait(bound) => {
                    let last = bound.path.segments.last_mut().unwrap();
                    last.ident = format_ident!("{}Factory", last.ident);
                },
                _ => {},
            }
        }
    });
    fact.ident = fact_ident;
    let (fact_trait, fact_phantom) = get_full_name(&fact);
    let mut fact_items = vec![];
    for item in &mut inst.items {
        match item {
            syn::TraitItem::Fn(inst_fn) => {
                // inst_fn.default = None;
                let recv_arg = inst_fn.sig.receiver().map(|_| quote! { self });
                if recv_arg.is_none() {
                    inst_fn.sig.inputs.insert(0, syn::parse_quote! { &self });
                }
                let (args, pat) = inst_fn.sig.inputs
                    .iter_mut()
                    .filter_map(|arg| {
                        match arg {
                            syn::FnArg::Typed(arg) => Some(arg),
                            syn::FnArg::Receiver(recv) => {
                                if recv.ty.to_token_stream().to_string() == "Self" {
                                    recv.ty = syn::parse_quote! { Box<Self> };
                                }
                                None
                            },
                        }
                    })
                    .enumerate()
                    .map(|(i, arg)| {
                        let occurrence = Occurrence::substitute(&mut arg.ty, &generics, false, true, 0);
                        let ident = format_ident!("v{}", i + 1);
                        let (body, mutability) = occurrence.transform_input_unwrap(quote! { #ident });
                        let pat: syn::Pat = match mutability {
                            true => syn::parse_quote! { mut #ident },
                            false => syn::parse_quote! { #ident },
                        };
                        (body, pat)
                    })
                    .unzip::<TokenStream, syn::Pat, Vec<_>, Vec<_>>();
                let output = match &mut inst_fn.sig.output {
                    syn::ReturnType::Type(_, ty) => {
                        Occurrence::substitute(ty.as_mut(), &generics, false, false, 0)
                    },
                    _ => Occurrence::None,
                };
                let mut impl_fn = inst_fn.clone();
                impl_fn.attrs.push(syn::parse_quote! { #[inline] });
                let ident = &impl_fn.sig.ident;
                impl_fn.sig.inputs
                    .iter_mut()
                    .filter_map(|arg| {
                        match arg {
                            syn::FnArg::Typed(arg) => Some(arg),
                            syn::FnArg::Receiver(_) => None,
                        }
                    })
                    .zip(pat.into_iter())
                    .for_each(|(arg, pat)| {
                        arg.pat = Box::new(pat);
                    });
                let (body, _) = output.transform_output_unwrap(match recv_arg {
                    Some(_) => quote! { self.0.#ident(#(#args),*) },
                    None => quote! { Factory::#ident(#(#args),*) },
                });
                impl_fn.default = Some(syn::parse_quote! {{ #body }});
                fact_items.push(impl_fn);
            },
            _ => {},
        }
    }
    let mut fact_generics = fact.generics.clone();
    fact_generics.params.push(syn::parse_quote! { Factory: #fact_trait });
    let (impl_generics, _, where_clause) = fact_generics.split_for_impl();
    let fact = &fact;
    let inst = &inst;
    quote! {
        #inst
        #super_impls
        #fact
        #[automatically_derived]
        impl #impl_generics #inst_trait for ::dyn_std::Instance<Factory, #fact_phantom> #where_clause {
            #(#fact_items)*
        }
    }
}

#[derive(Debug, Clone)]
enum RefType {
    Ref,
    Mut,
    Box,
}

#[derive(Debug, Clone)]
enum FnType {
    Fn,
    FnMut,
    FnOnce,
}

#[derive(Debug, Clone)]
enum Occurrence {
    Exact(TokenStream),
    Args(Vec<(Occurrence, syn::Type, syn::Type)>),
    Tuple(Vec<Occurrence>),
    RefLike(Box<Occurrence>, RefType),
    Fn(FnType, TokenStream, TokenStream, Box<Occurrence>),
    None,
}

impl Occurrence {
    fn substitute(
        ty: &mut syn::Type,
        generics: &GenericsData,
        is_ref: bool,
        polarity: bool,
        depth: usize,
    ) -> Self {
        match ty {
            syn::Type::Path(tp) => {
                if tp.qself.is_none() && tp.path.segments.len() == 1 {
                    let last = tp.path.segments.last_mut().unwrap();
                    if last.ident == "Box" {
                        let syn::PathArguments::AngleBracketed(args) = &mut last.arguments else {
                            panic!("expected angle-bracketed arguments in Box type")
                        };
                        if args.args.len() != 1 {
                            panic!("expected exactly one argument in Box type")
                        }
                        let syn::GenericArgument::Type(ty) = args.args.first_mut().unwrap() else {
                            panic!("expected type argument in Box type")
                        };
                        let o = Self::substitute(ty, generics, true, polarity, depth);
                        if matches!(o, Self::None) {
                            return Self::None
                        } else {
                            return Self::RefLike(Box::new(o), RefType::Box)
                        }
                    }
                }
                let result = generics.test(tp, is_ref);
                if let Some((repl, repl2)) = result {
                    *ty = repl;
                    return Self::Exact(repl2)
                }
                let syn::PathArguments::AngleBracketed(args) = &mut tp.path.segments.last_mut().unwrap().arguments else {
                    return Self::None
                };
                let args = args.args
                    .iter_mut()
                    .filter_map(|arg| {
                        let syn::GenericArgument::Type(ty) = arg else {
                            return None
                        };
                        let mut old_ty = ty.clone();
                        subst_self(&mut old_ty, &syn::parse_quote! { Factory });
                        let o = Self::substitute(ty, generics, false, polarity, depth);
                        Some((o, old_ty, ty.clone()))
                    })
                    .collect::<Vec<_>>();
                if args.iter().all(|(o, ..)| matches!(o, Self::None)) {
                    return Self::None
                } else {
                    return Self::Args(args)
                }
            },
            syn::Type::Tuple(tt) => {
                let mut nothing = true;
                let mut ts = tt.elems.iter_mut().collect::<Vec<_>>();
                let os = ts.iter_mut().map(|ty| {
                    let o = Self::substitute(ty, generics, false, polarity, depth);
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
                let o = Self::substitute(&mut tr.elem, generics, true, polarity, depth);
                if matches!(o, Self::None) {
                    return Self::None
                } else {
                    return Self::RefLike(Box::new(o), match tr.mutability {
                        Some(_) => RefType::Mut,
                        None => RefType::Ref,
                    })
                }
            },
            syn::Type::Slice(ts) => {
                let o = Self::substitute(&mut ts.elem, generics, false, polarity, depth);
                if matches!(o, Self::None) {
                    return Self::None
                } else {
                    unimplemented!("slice types in trait methods")
                }
            },
            syn::Type::Ptr(ptr) => {
                let o = Self::substitute(&mut ptr.elem, generics, false, polarity, depth);
                if matches!(o, Self::None) {
                    return Self::None
                } else {
                    unimplemented!("pointers in trait methods")
                }
            },
            syn::Type::ImplTrait(_) => unimplemented!("impl trait in trait methods"),
            syn::Type::TraitObject(trait_object) => {
                for bound in &mut trait_object.bounds {
                    let syn::TypeParamBound::Trait(bound) = bound else {
                        continue;
                    };
                    if bound.path.segments.len() != 1 {
                        continue;
                    }
                    let last = bound.path.segments.last_mut().unwrap();
                    let fn_type = match last.ident.to_string().as_str() {
                        "Fn" => FnType::Fn,
                        "FnMut" => FnType::FnMut,
                        "FnOnce" => FnType::FnOnce,
                        _ => continue,
                    };
                    let syn::PathArguments::Parenthesized(args) = &mut last.arguments else {
                        continue;
                    };
                    let inputs = args.inputs
                        .iter_mut()
                        .map(|ty| Self::substitute(ty, generics, false, !polarity, depth + 1))
                        .collect::<Vec<_>>();
                    let output = match &mut args.output {
                        syn::ReturnType::Type(_, ty) => {
                            Occurrence::substitute(ty.as_mut(), generics, false, polarity, depth + 1)
                        },
                        _ => Occurrence::None,
                    };
                    if inputs.iter().all(|input| matches!(input, Occurrence::None)) && matches!(output, Occurrence::None) {
                        continue;
                    }
                    let (exprs, args) = inputs.into_iter().enumerate().map(|(i, occurrence)| {
                        let ident = format_ident!("v{}_{}", depth + 1, i + 1);
                        let (expr, mutability) = match polarity {
                            true => occurrence.transform_output_unwrap(quote! { #ident }),
                            false => occurrence.transform_input_unwrap(quote! { #ident }),
                        };
                        let prefix = make_mut_prefix(mutability);
                        (expr, quote! { #prefix #ident })
                    }).unzip::<TokenStream, TokenStream, Vec<_>, Vec<_>>();
                    // fixme: more than one trait
                    return Occurrence::Fn(fn_type, quote! { #(#args),* }, quote! { #(#exprs),* }, Box::new(output))
                }
                Occurrence::None
            },
            _ => Self::None,
        }
    }

    fn transform_input_unwrap(&self, expr: TokenStream) -> (TokenStream, bool) {
        self.transform_input(&expr).unwrap_or((expr, false))
    }

    fn transform_input(&self, expr: &TokenStream) -> Option<(TokenStream, bool)> {
        match self {
            Occurrence::None => None,
            Occurrence::Exact(ty) => Some((quote! { #ty::downcast(#expr) }, false)),
            Occurrence::RefLike(o, ref_type) => match o.as_ref() {
                Occurrence::Exact(ty) => Some((match ref_type {
                    RefType::Box => quote! { Box::new(#ty::downcast(#expr)) },
                    RefType::Mut => quote! { #ty::downcast_mut(#expr) },
                    RefType::Ref => quote! { #ty::downcast_ref(#expr) },
                }, false)),
                Occurrence::Fn(..) => {
                    let (expr, mutability) = o.transform_input(expr).unwrap();
                    Some(match ref_type {
                        RefType::Box => (quote! { Box::new(move #expr) }, mutability),
                        RefType::Mut => (quote! { &mut #expr }, false),
                        RefType::Ref => (quote! { & #expr }, false),
                    })
                },
                // Currently supported reference types:
                // - &T (where T does not contain Self)
                // - &dyn Fn() (where parameter and return types are all non-referencing valid parameter types)
                // TODO: support box types
                _ => unimplemented!("reference in trait method param type"),
            },
            Occurrence::Args(args) => {
                let len = args.len();
                let ident = format_ident!("Map{}", len);
                let args = args.iter().map(|(o, old, new)| {
                    let (expr, mutability) = o.transform_input_unwrap(quote! { x });
                    let prefix = make_mut_prefix(mutability);
                    quote! { |#prefix x: #new| -> #old { #expr } }
                });
                Some((quote! { ::dyn_std::map::#ident::map(#expr, #(#args),*) }, false))
            },
            Occurrence::Tuple(os) => {
                let (idents, values) = os.iter().enumerate().map(|(i, o)| {
                    let ident = format_ident!("v{}", i + 1);
                    let (expr, mutability) = o.transform_input_unwrap(quote! { #ident });
                    let prefix = make_mut_prefix(mutability);
                    (quote! { #prefix #ident }, expr)
                }).unzip::<TokenStream, TokenStream, Vec<_>, Vec<_>>();
                Some((quote! {
                    match #expr {
                        (#(#idents),*) => (#(#values),*)
                    }
                }, false))
            },
            Occurrence::Fn(fn_type, args, exprs, output) => {
                let (body, _mutability) = output.transform_input_unwrap(quote! { #expr(#exprs) });
                Some((quote! { |#args| #body }, matches!(fn_type, FnType::FnMut)))
            },
        }
    }

    fn transform_output_unwrap(&self, expr: TokenStream) -> (TokenStream, bool) {
        self.transform_output(&expr).unwrap_or((expr, false))
    }

    fn transform_output(&self, expr: &TokenStream) -> Option<(TokenStream, bool)> {
        match self {
            Occurrence::Exact(_) => Some((quote! { Box::new(::dyn_std::Instance::new(#expr)) }, false)),
            Occurrence::None => None,
            Occurrence::Args(args) => {
                let len = args.len();
                let ident = format_ident!("Map{}", len);
                let args = args.iter().map(|(o, old, new)| {
                    let (expr, mutability) = o.transform_output_unwrap(quote! { x });
                    match mutability {
                        true => quote! { |mut x: #old| -> #new { #expr } },
                        false => quote! { |x: #old| -> #new { #expr } },
                    }
                });
                Some((quote! { ::dyn_std::map::#ident::map(#expr, #(#args),*) }, false))
            },
            Occurrence::Tuple(os) => {
                let (idents, values) = os.iter().enumerate().map(|(i, o)| {
                    let ident = format_ident!("v{}", i + 1);
                    let (expr, mutability) = o.transform_output_unwrap(quote! { #ident });
                    let prefix = make_mut_prefix(mutability);
                    (quote! { #prefix #ident }, expr)
                }).unzip::<TokenStream, TokenStream, Vec<_>, Vec<_>>();
                Some((quote! {
                    match #expr {
                        (#(#idents),*) => (#(#values),*)
                    }
                }, false))
            },
            Occurrence::RefLike(o, ref_type) => {
                let RefType::Box = ref_type else {
                    unimplemented!("reference in trait method return type")
                };
                let (expr, mutability) = o.transform_output(expr).unwrap();
                Some((quote! { Box::new(move #expr) }, mutability))
            },
            Occurrence::Fn(fn_type, args, exprs, output) => {
                let (body, _mutability) = output.transform_output_unwrap(quote! { #expr(#exprs) });
                Some((quote! { |#args| #body }, matches!(fn_type, FnType::FnMut)))
            },
        }
    }
}
