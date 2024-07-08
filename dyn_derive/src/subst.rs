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
    inline: bool,
    arg_pat: Option<(TokenStream, usize)>,
}

impl Clone for Context<'_> {
    fn clone(&self) -> Self {
        Self {
            generics: self.generics,
            polarity: self.polarity,
            depth: self.depth,
            // clear
            ref_type: RefType::None,
            is_dirty: false,
            inline: false,
            arg_pat: None,
        }
    }
}

impl<'i> Context<'i> {
    pub fn new(generics: &'i GenericsData) -> Self {
        Self {
            generics,
            ref_type: RefType::None,
            polarity: false,
            depth: 0,
            is_dirty: false,
            inline: false,
            arg_pat: None,
        }
    }

    pub fn subst_fn<'j>(&mut self, inputs: impl Iterator<Item = &'j mut syn::Type>, output: &mut syn::ReturnType, expr: &impl ToTokens) -> (TokenStream, TokenStream, Vec<TokenStream>) {
        let mut params = vec![];
        let mut exprs = vec![];
        let mut stmts = quote![];
        let mut i: usize = 0;
        for ty in inputs {
            i += 1;
            let ident = if self.depth == 0 {
                format_ident!("v{}", i)
            } else {
                format_ident!("v{}_{}", self.depth, i)
            };
            let mut ctx = self.clone();
            ctx.polarity ^= true;
            ctx.depth += 1;
            ctx.arg_pat = Some((quote! { #ident }, i));
            let (expr, inner_stmts, mutability) = ctx.subst(ty, &ident);
            self.is_dirty |= ctx.is_dirty;
            stmts.extend(inner_stmts);
            let prefix = make_mut_prefix(mutability);
            if ctx.is_dirty && !ctx.inline {
                params.push(quote! { #ident });
                exprs.push(quote! { #ident });
                stmts.extend(quote! { let #prefix #ident = #expr; });
            } else {
                exprs.push(expr);
                params.push(quote! { #prefix #ident });
            }
        }
        let expr = quote! { #expr(#(#exprs),*) };
        let mut ctx = self.clone();
        ctx.depth += 1;
        let (expr, inner_stmts, _) = match output {
            syn::ReturnType::Type(_, ty) => ctx.subst(ty, &expr),
            syn::ReturnType::Default => (expr.to_token_stream(), quote!{}, false),
        };
        self.is_dirty |= ctx.is_dirty;
        stmts.extend(inner_stmts);
        (expr, stmts, params)
    }

    pub fn subst(&mut self, ty: &mut syn::Type, expr: &impl ToTokens) -> (TokenStream, TokenStream, bool) {
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
                        let mut ctx = self.clone();
                        ctx.ref_type = RefType::Box;
                        let result = ctx.subst(ty, expr);
                        if !ctx.is_dirty {
                            break 'k
                        }
                        if self.ref_type != RefType::None {
                            unimplemented!("nested reference in trait method")
                        }
                        self.is_dirty = true;
                        return result
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
                    }, quote![], false)
                }
                let syn::PathArguments::AngleBracketed(args) = &mut tp.path.segments.last_mut().unwrap().arguments else {
                    break 'k
                };
                let mut ctx = self.clone();
                let args = args.args
                    .iter_mut()
                    .filter_map(|arg| {
                        let syn::GenericArgument::Type(ty_inst) = arg else {
                            return None
                        };
                        let mut ty_cons = ty_inst.clone();
                        subst_self(&mut ty_cons, &syn::parse_quote! { Factory });
                        let (expr, stmts, mutability) = ctx.subst(ty_inst, &quote! { x });
                        let prefix = make_mut_prefix(mutability);
                        Some(if self.polarity {
                            quote! { |#prefix x: #ty_inst| -> #ty_cons { #stmts #expr } }
                        } else {
                            quote! { |#prefix x: #ty_cons| -> #ty_inst { #stmts #expr } }
                        })
                    })
                    .collect::<Vec<_>>();
                if !ctx.is_dirty {
                    break 'k
                }
                let len = args.len();
                let ident = format_ident!("Map{}", len);
                self.is_dirty = true;
                return (quote! { ::dyn_std::map::#ident::map(#expr, #(#args),*) }, quote![], false)
            },
            syn::Type::Reference(tr) => 'k: {
                let mut ctx = self.clone();
                ctx.ref_type = match tr.mutability {
                    Some(_) => RefType::Mut,
                    None => RefType::Ref,
                };
                let result = ctx.subst(&mut tr.elem, expr);
                if !ctx.is_dirty {
                    break 'k
                }
                if self.ref_type != RefType::None {
                    unimplemented!("nested reference in trait method")
                }
                self.is_dirty = true;
                return result
            },
            syn::Type::Tuple(tuple) => 'k: {
                let mut stmts = quote![];
                let (pats, exprs) = tuple.elems.iter_mut().enumerate().map(|(i, ty)| {
                    let ident = format_ident!("v{}", i + 1);
                    let mut ctx = self.clone();
                    let (expr, inner_stmts, mutability) = ctx.subst(ty, &ident);
                    self.is_dirty |= ctx.is_dirty;
                    stmts.extend(inner_stmts);
                    let prefix = make_mut_prefix(mutability);
                    if ctx.is_dirty && !ctx.inline {
                        stmts.extend(quote! { let #prefix #ident = #expr; });
                        (quote! { #ident }, quote! { #ident })
                    } else {
                        (quote! { #prefix #ident }, expr)
                    }
                }).unzip::<TokenStream, TokenStream, Vec<_>, Vec<_>>();
                if !self.is_dirty {
                    break 'k
                }
                self.is_dirty = true;
                self.inline = true;
                return (quote! {
                    (#(#exprs),*)
                }, quote! {
                    let (#(#pats),*) = #expr;
                    #stmts
                }, false)
            },
            syn::Type::Slice(slice) => {
                let mut ctx = self.clone();
                let _ = ctx.subst(&mut slice.elem, expr);
                if ctx.is_dirty {
                    unimplemented!("slice types in trait methods")
                }
            },
            syn::Type::Ptr(ptr) => {
                let mut ctx = self.clone();
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
                    let (expr, stmts, params) = self.subst_fn(args.inputs.iter_mut(), &mut args.output, expr);
                    if !self.is_dirty {
                        break 'k
                    }
                    let closure = if stmts.is_empty() {
                        quote! { |#(#params),*| #expr }
                    } else {
                        quote! { |#(#params),*| { #stmts #expr } }
                    };
                    return match self.ref_type {
                        RefType::Box => (quote! { Box::new(move #closure) }, quote![], matches!(fn_type, FnTrait::FnMut)),
                        RefType::Mut => (quote! { &mut #closure }, quote![], false),
                        RefType::Ref => (quote! { & #closure }, quote![], false),
                        RefType::None => unreachable!("expect &dyn, &mut dyn or Box<dyn>"),
                    }
                }
            },
            _ => {},
        }
        (expr.to_token_stream(), quote![], false)
    }
}
