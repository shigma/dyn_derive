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

pub enum Destruct {
    Preserve(TokenStream),
    Tuple(TokenStream),
}

impl Default for Destruct {
    fn default() -> Self {
        Self::Preserve(quote! {})
    }
}

pub struct Context<'i> {
    generics: &'i GenericsData,
    ref_type: RefType,
    polarity: bool,
    depth: usize,
}

impl Clone for Context<'_> {
    fn clone(&self) -> Self {
        Self {
            generics: self.generics,
            polarity: self.polarity,
            depth: self.depth,
            // clear ref
            ref_type: RefType::None,
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
        }
    }

    fn subst_ident(&self, ty: &mut syn::Type, stmts: &mut TokenStream, offset: &mut usize) -> (TokenStream, TokenStream, bool) {
        let char = (b'a' + (self.depth as u8 - 1)) as char;
        let ident = format_ident!("{}{}", char, *offset + 1);
        let (expr, inner_stmts, destruct, has_match) = self.subst(ty, &ident, offset);
        stmts.extend(inner_stmts);
        match destruct {
            Destruct::Preserve(pref) => {
                if has_match {
                    stmts.extend(quote! { let #pref #ident = #expr; });
                    (quote! { #ident }, quote! { #ident }, has_match)
                } else {
                    (quote! { #pref #ident }, expr, has_match)
                }
            },
            Destruct::Tuple(pat) => {
                *offset -= 1;
                (quote! { #pat }, expr, has_match)
            },
        }
    }

    pub fn subst_fn<'j>(&self, inputs: impl Iterator<Item = &'j mut syn::Type>, output: &mut syn::ReturnType, expr: &impl ToTokens) -> (TokenStream, TokenStream, Vec<TokenStream>, bool) {
        let mut params = vec![];
        let mut exprs = vec![];
        let mut stmts = quote![];
        let mut offset = 0;
        let mut has_match = false;
        for ty in inputs {
            let mut ctx = self.clone();
            ctx.polarity ^= true;
            ctx.depth += 1;
            let (pat, expr, has_input_match) = ctx.subst_ident(ty, &mut stmts, &mut offset);
            println!("{} {}", ty.to_token_stream().to_string(), has_input_match);
            has_match |= has_input_match;
            offset += 1;
            exprs.push(expr);
            params.push(pat);
        }
        let expr = quote! { #expr(#(#exprs),*) };
        let mut ctx = self.clone();
        ctx.depth += 1;
        let (new_expr, inner_stmts, destruct, has_output_match) = match output {
            syn::ReturnType::Type(_, ty) => ctx.subst(ty, &expr, &mut offset),
            syn::ReturnType::Default => (expr.to_token_stream(), quote! {}, Default::default(), false),
        };
        if let Destruct::Tuple(pat) = destruct {
            stmts.extend(quote! { let #pat = #expr; });
        }
        has_match |= has_output_match;
        stmts.extend(inner_stmts);
        (new_expr, stmts, params, has_match)
    }

    fn subst_map<'j>(&self, inputs: impl Iterator<Item = &'j mut syn::Type>, expr: &impl ToTokens) -> (TokenStream, TokenStream, Destruct, bool) {
        let mut has_match = false;
        let args = inputs.map(|ty_inst| {
            let mut ty_cons = ty_inst.clone();
            subst_self(&mut ty_cons, &syn::parse_quote! { Factory });
            let mut ctx = self.clone();
            ctx.depth += 1;
            let (expr, stmts, destruct, has_inner) = ctx.subst(ty_inst, &quote! { x }, &mut 0);
            let pat = match destruct {
                Destruct::Preserve(modifier) => quote! { #modifier x },
                Destruct::Tuple(pat) => pat,
            };
            has_match |= has_inner;
            Some(match has_inner {
                true => match self.polarity {
                    true => quote! { |#pat: #ty_inst| -> #ty_cons { #stmts #expr } },
                    false => quote! { |#pat: #ty_cons| -> #ty_inst { #stmts #expr } },
                },
                false => {
                    assert_eq!(ty_inst, &ty_cons);
                    assert!(stmts.is_empty());
                    quote! { |x: #ty_inst| x }
                },
            })
        }).collect::<Vec<_>>();
        let len = args.len();
        let ident = format_ident!("Map{}", len);
        return (quote! { ::dyn_std::map::#ident::map(#expr, #(#args),*) }, quote![], Default::default(), has_match)
    }

    pub fn subst(&self, ty: &mut syn::Type, expr: &impl ToTokens, offset: &mut usize) -> (TokenStream, TokenStream, Destruct, bool) {
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
                        let result = ctx.subst(ty, expr, offset);
                        if !result.3 {
                            break 'k
                        }
                        if self.ref_type != RefType::None {
                            unimplemented!("nested reference in trait method")
                        }
                        return result
                    }
                }
                let result = self.generics.test(tp, self.ref_type != RefType::None);
                if let Some((repl, repl2)) = result {
                    *ty = repl;
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
                    }, quote![], Default::default(), true)
                }
                let syn::PathArguments::AngleBracketed(args) = &mut tp.path.segments.last_mut().unwrap().arguments else {
                    break 'k
                };
                let args = args.args
                    .iter_mut()
                    .filter_map(|arg| {
                        match arg {
                            syn::GenericArgument::Type(ty) => Some(ty),
                            _ => None,
                        }
                    });
                let result = self.subst_map(args, expr);
                if !result.3 {
                    break 'k
                }
                return result
            },
            syn::Type::Reference(reference) => 'k: {
                let mut ctx = self.clone();
                ctx.ref_type = match reference.mutability {
                    Some(_) => RefType::Mut,
                    None => RefType::Ref,
                };
                let result = ctx.subst(&mut reference.elem, expr, offset);
                if !result.3 {
                    break 'k
                }
                if self.ref_type != RefType::None {
                    unimplemented!("nested reference in trait method")
                }
                return result
            },
            syn::Type::Tuple(tuple) => 'k: {
                let mut stmts = quote![];
                let mut has_match = false;
                let (pats, exprs) = tuple.elems.iter_mut().map(|ty| {
                    let ctx = self.clone();
                    let (pat, expr, has_inner_match) = ctx.subst_ident(ty, &mut stmts, offset);
                    *offset += 1;
                    has_match |= has_inner_match;
                    (pat, expr)
                }).unzip::<TokenStream, TokenStream, Vec<_>, Vec<_>>();
                if !has_match {
                    break 'k
                }
                return (quote! { (#(#exprs),*) }, stmts, Destruct::Tuple(quote! { (#(#pats),*) }), true)
            },
            syn::Type::Array(array) => 'k: {
                let result = self.subst_map([array.elem.as_mut()].into_iter(), expr);
                if !result.3 {
                    break 'k
                }
                return result
            },
            syn::Type::Slice(slice) => {
                let ctx = self.clone();
                let result = ctx.subst(&mut slice.elem, expr, offset);
                if result.3 {
                    unimplemented!("slice types in trait methods")
                }
            },
            syn::Type::Ptr(ptr) => {
                let ctx = self.clone();
                let result = ctx.subst(&mut ptr.elem, expr, offset);
                if result.3 {
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
                    let (expr, stmts, params, has_match) = self.subst_fn(args.inputs.iter_mut(), &mut args.output, expr);
                    if !has_match {
                        break 'k
                    }
                    let closure = if stmts.is_empty() {
                        quote! { |#(#params),*| #expr }
                    } else {
                        quote! { |#(#params),*| { #stmts #expr } }
                    };
                    return match self.ref_type {
                        RefType::Box => (quote! { Box::new(move #closure) }, quote![], Destruct::Preserve(match fn_type {
                            FnTrait::FnMut => quote! { mut },
                            _ => quote! {},
                        }), true),
                        RefType::Mut => (quote! { &mut #closure }, quote![], Default::default(), true),
                        RefType::Ref => (quote! { & #closure }, quote![], Default::default(), true),
                        RefType::None => unreachable!("expect &dyn, &mut dyn or Box<dyn>"),
                    }
                }
            },
            _ => {},
        }
        (expr.to_token_stream(), quote![], Default::default(), false)
    }
}
