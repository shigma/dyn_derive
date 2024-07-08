use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::{generics::GenericsData, subst_self::subst_self};

#[derive(Debug, Clone, PartialEq, Eq)]
enum FnTrait {
    Fn,
    FnMut,
    FnOnce,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RefType {
    Ref,
    Mut,
    Box,
    None,
}

fn make_mut_prefix(mutability: bool) -> TokenStream {
    if mutability {
        quote! { mut }
    } else {
        quote! {}
    }
}

pub struct Context<'i> {
    generics: &'i GenericsData,
    ref_type: RefType,
    polarity: bool,
    depth: usize,
    is_dirty: bool,
}

impl<'i> Context<'i> {
    pub fn new(generics: &'i GenericsData) -> Self {
        Self {
            generics,
            ref_type: RefType::None,
            polarity: false,
            depth: 0,
            is_dirty: false,
        }
    }

    fn fork_deep(&self, polarity: bool) -> Self {
        Self {
            generics: self.generics,
            ref_type: RefType::None,
            polarity: self.polarity ^ polarity,
            depth: self.depth + 1,
            is_dirty: false,
        }
    }

    fn fork_ref(&self, ref_type: RefType) -> Self {
        Self {
            generics: self.generics,
            ref_type,
            polarity: self.polarity,
            depth: self.depth,
            is_dirty: false,
        }
    }

    fn fork(&self) -> Self {
        self.fork_ref(RefType::None)
    }

    pub fn subst_fn<'j>(&self, inputs: impl Iterator<Item = &'j mut syn::Type>, output: &mut syn::ReturnType, expr: &impl ToTokens) -> (TokenStream, Vec<TokenStream>, bool) {
        let mut ctx_input = self.fork_deep(true);
        let (params, exprs) = ctx_input.subst_many(inputs);
        let expr = quote! { #expr(#(#exprs),*) };
        let mut ctx_output = self.fork_deep(false);
        let (expr, _) = match output {
            syn::ReturnType::Type(_, ty) => ctx_output.subst(ty, &expr),
            syn::ReturnType::Default => (expr.to_token_stream(), false),
        };
        (expr, params, ctx_input.is_dirty || ctx_output.is_dirty)
    }

    fn subst_many<'j>(&mut self, tys: impl Iterator<Item = &'j mut syn::Type>) -> (Vec<TokenStream>, Vec<TokenStream>) {
        let (params, exprs) = tys
            .enumerate()
            .map(|(i, ty)| {
                let ident = if self.depth == 1 {
                    format_ident!("v{}", i + 1)
                } else {
                    format_ident!("v{}_{}", self.depth - 1, i + 1)
                };
                let (expr, mutability) = self.subst(ty, &ident);
                let prefix = make_mut_prefix(mutability);
                (quote! { #prefix #ident }, expr)
            })
            .unzip::<TokenStream, TokenStream, Vec<_>, Vec<_>>();
        (params, exprs)
    }

    pub fn subst(
        &mut self,
        ty: &mut syn::Type,
        expr: &impl ToTokens,
    ) -> (TokenStream, bool) {
        match ty {
            syn::Type::Path(tp) => 'k: {
                if tp.qself.is_none() && tp.path.segments.len() == 1 {
                    let last = tp.path.segments.last_mut().unwrap();
                    if last.ident == "Box" {
                        let syn::PathArguments::AngleBracketed(args) = &mut last.arguments else {
                            panic!("expect angle-bracketed arguments in Box type")
                        };
                        if args.args.len() != 1 {
                            panic!("expect exactly one argument in Box type")
                        }
                        let syn::GenericArgument::Type(ty) = args.args.first_mut().unwrap() else {
                            panic!("expect type argument in Box type")
                        };
                        let mut ctx = self.fork_ref(RefType::Box);
                        let (expr, mutability) = ctx.subst(ty, expr);
                        if !ctx.is_dirty {
                            break 'k
                        }
                        if self.ref_type != RefType::None {
                            unimplemented!("nested reference in trait method")
                        }
                        self.is_dirty = true;
                        return (expr, mutability)
                    }
                }
                let result = self.generics.test(tp, self.ref_type != RefType::None);
                if let Some((repl, repl2)) = result {
                    *ty = repl;
                    self.is_dirty = true;
                    return (if self.polarity {
                        match self.ref_type {
                            RefType::Mut => quote! { #repl2::downcast_mut(#expr) },
                            RefType::Ref => quote! { #repl2::downcast_ref(#expr) },
                            RefType::Box => quote! { Box::new(#repl2::downcast(#expr)) },
                            RefType::None => quote! { #repl2::downcast(#expr) },
                        }
                    } else {
                        match self.ref_type {
                            RefType::None => quote! { Box::new(::dyn_std::Instance::new(#expr)) },
                            _ => unimplemented!("reference in trait method return type"),
                        }
                    }, false)
                }
                let syn::PathArguments::AngleBracketed(args) = &mut tp.path.segments.last_mut().unwrap().arguments else {
                    break 'k
                };
                let mut fork = self.fork();
                let args = args.args
                    .iter_mut()
                    .filter_map(|arg| {
                        let syn::GenericArgument::Type(ty) = arg else {
                            return None
                        };
                        let mut old = ty.clone();
                        subst_self(&mut old, &syn::parse_quote! { Factory });
                        let (expr, mutability) = fork.subst(ty, &quote! { x });
                        let prefix = make_mut_prefix(mutability);
                        Some(if self.polarity {
                            quote! { |#prefix x: #ty| -> #old { #expr } }
                        } else {
                            quote! { |#prefix x: #old| -> #ty { #expr } }
                        })
                    })
                    .collect::<Vec<_>>();
                if !fork.is_dirty {
                    break 'k
                }
                let len = args.len();
                let ident = format_ident!("Map{}", len);
                self.is_dirty = true;
                return (quote! { ::dyn_std::map::#ident::map(#expr, #(#args),*) }, false)
            },
            syn::Type::Reference(tr) => 'k: {
                let mut ctx = self.fork_ref(match tr.mutability {
                    Some(_) => RefType::Mut,
                    None => RefType::Ref,
                });
                let (expr, mutability) = ctx.subst(&mut tr.elem, expr);
                if !ctx.is_dirty {
                    break 'k
                }
                if self.ref_type != RefType::None {
                    unimplemented!("nested reference in trait method")
                }
                self.is_dirty = true;
                return (expr, mutability)
            },
            syn::Type::Tuple(tuple) => 'k: {
                let mut ctx = self.fork();
                let (idents, values) = tuple.elems.iter_mut().enumerate().map(|(i, ty)| {
                    let ident = format_ident!("v{}", i + 1);
                    let (expr, mutability) = ctx.subst(ty, &ident);
                    let prefix = make_mut_prefix(mutability);
                    (quote! { #prefix #ident }, expr)
                }).unzip::<TokenStream, TokenStream, Vec<_>, Vec<_>>();
                if !ctx.is_dirty {
                    break 'k
                }
                self.is_dirty = true;
                return (quote! {
                    match #expr {
                        (#(#idents),*) => (#(#values),*)
                    }
                }, false)
            },
            syn::Type::Slice(slice) => {
                let mut ctx = self.fork();
                let _ = ctx.subst(&mut slice.elem, expr);
                if ctx.is_dirty {
                    unimplemented!("slice types in trait methods")
                }
            },
            syn::Type::Ptr(ptr) => {
                let mut ctx = self.fork();
                let _ = ctx.subst(&mut ptr.elem, expr);
                if ctx.is_dirty {
                    unimplemented!("pointers in trait methods")
                }
            },
            syn::Type::ImplTrait(_) => {
                unimplemented!("impl trait in trait methods")
            },
            syn::Type::TraitObject(trait_object) => 'k: {
                for bound in &mut trait_object.bounds {
                    let syn::TypeParamBound::Trait(bound) = bound else {
                        continue;
                    };
                    if bound.path.segments.len() != 1 {
                        continue;
                    }
                    let last = bound.path.segments.last_mut().unwrap();
                    let fn_type = match last.ident.to_string().as_str() {
                        "Fn" => FnTrait::Fn,
                        "FnMut" => FnTrait::FnMut,
                        "FnOnce" => FnTrait::FnOnce,
                        _ => continue,
                    };
                    let syn::PathArguments::Parenthesized(args) = &mut last.arguments else {
                        panic!("expect parenthesized arguments in {} trait", last.ident)
                    };
                    let (expr, params, is_dirty) = self.subst_fn(args.inputs.iter_mut(), &mut args.output, expr);
                    if !is_dirty {
                        break 'k
                    }
                    self.is_dirty = true;
                    return match self.ref_type {
                        RefType::Box => (quote! { Box::new(move |#(#params),*| #expr) }, matches!(fn_type, FnTrait::FnMut)),
                        RefType::Mut => (quote! { &mut |#(#params),*| #expr }, false),
                        RefType::Ref => (quote! { & |#(#params),*| #expr }, false),
                        RefType::None => unreachable!("expect &dyn, &mut dyn or Box<dyn>"),
                    }
                }
            },
            _ => {},
        }
        (expr.to_token_stream(), false)
    }
}
