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
    is_dirty: bool,
    inline: bool,
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
        }
    }

    fn subst_ident(&mut self, ty: &mut syn::Type, ident: &syn::Ident, stmts: &mut TokenStream, offset: &mut usize) -> (TokenStream, TokenStream) {
        let (expr, inner_stmts, destruct) = self.subst(ty, &ident, offset);
        stmts.extend(inner_stmts);
        match destruct {
            Destruct::Preserve(pref) => {
                if self.is_dirty {
                    stmts.extend(quote! { let #pref #ident = #expr; });
                    (quote! { #ident }, quote! { #ident })
                } else {
                    (quote! { #pref #ident }, expr)
                }
            },
            Destruct::Tuple(pat) => {
                *offset -= 1;
                (quote! { #pat }, expr)
            },
        }
    }

    fn format_ident(&self, offset: usize) -> syn::Ident {
        let char = (b'a' + (self.depth as u8 - 1)) as char;
        format_ident!("{}{}", char, offset + 1)
    }

    pub fn subst_fn<'j>(&mut self, inputs: impl Iterator<Item = &'j mut syn::Type>, output: &mut syn::ReturnType, expr: &impl ToTokens) -> (TokenStream, TokenStream, Vec<TokenStream>) {
        let mut params = vec![];
        let mut exprs = vec![];
        let mut stmts = quote![];
        let mut offset = 0;
        for ty in inputs {
            let mut ctx = self.clone();
            ctx.polarity ^= true;
            ctx.depth += 1;
            let ident = ctx.format_ident(offset);
            let (pat, expr) = ctx.subst_ident(ty, &ident, &mut stmts, &mut offset);
            self.is_dirty |= ctx.is_dirty;
            offset += 1;
            exprs.push(expr);
            params.push(pat);
        }
        let expr = quote! { #expr(#(#exprs),*) };
        let mut ctx = self.clone();
        ctx.depth += 1;
        let (new_expr, inner_stmts, destruct) = match output {
            syn::ReturnType::Type(_, ty) => ctx.subst(ty, &expr, &mut offset),
            syn::ReturnType::Default => (expr.to_token_stream(), quote!{}, Default::default()),
        };
        if let Destruct::Tuple(pat) = destruct {
            stmts.extend(quote! { let #pat = #expr; });
        }
        self.is_dirty |= ctx.is_dirty;
        stmts.extend(inner_stmts);
        (new_expr, stmts, params)
    }

    pub fn subst(&mut self, ty: &mut syn::Type, expr: &impl ToTokens, offset: &mut usize) -> (TokenStream, TokenStream, Destruct) {
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
                    }, quote![], Default::default())
                }
                let syn::PathArguments::AngleBracketed(args) = &mut tp.path.segments.last_mut().unwrap().arguments else {
                    break 'k
                };
                let args = args.args
                    .iter_mut()
                    .filter_map(|arg| {
                        let syn::GenericArgument::Type(ty_inst) = arg else {
                            return None
                        };
                        let mut ty_cons = ty_inst.clone();
                        subst_self(&mut ty_cons, &syn::parse_quote! { Factory });
                        let mut ctx = self.clone();
                        let (expr, stmts, destruct) = ctx.subst(ty_inst, &quote! { x }, offset);
                        let pat = match destruct {
                            Destruct::Preserve(modifier) => quote! { #modifier x },
                            Destruct::Tuple(pat) => pat,
                        };
                        self.is_dirty |= ctx.is_dirty;
                        Some(match ctx.is_dirty {
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
                    })
                    .collect::<Vec<_>>();
                if !self.is_dirty {
                    break 'k
                }
                let len = args.len();
                let ident = format_ident!("Map{}", len);
                return (quote! { ::dyn_std::map::#ident::map(#expr, #(#args),*) }, quote![], Default::default())
            },
            syn::Type::Reference(reference) => 'k: {
                let mut ctx = self.clone();
                ctx.ref_type = match reference.mutability {
                    Some(_) => RefType::Mut,
                    None => RefType::Ref,
                };
                let result = ctx.subst(&mut reference.elem, expr, offset);
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
                let (pats, exprs) = tuple.elems.iter_mut().map(|ty| {
                    let ident = self.format_ident(*offset);
                    let mut ctx = self.clone();
                    let (pat, expr) = ctx.subst_ident(ty, &ident, &mut stmts, offset);
                    *offset += 1;
                    self.is_dirty |= ctx.is_dirty;
                    (pat, expr)
                }).unzip::<TokenStream, TokenStream, Vec<_>, Vec<_>>();
                if !self.is_dirty {
                    break 'k
                }
                self.is_dirty = true;
                self.inline = true;
                return (quote! { (#(#exprs),*) }, stmts, Destruct::Tuple(quote! { (#(#pats),*) }))
            },
            syn::Type::Slice(slice) => {
                let mut ctx = self.clone();
                ctx.subst(&mut slice.elem, expr, offset);
                if ctx.is_dirty {
                    unimplemented!("slice types in trait methods")
                }
            },
            syn::Type::Ptr(ptr) => {
                let mut ctx = self.clone();
                ctx.subst(&mut ptr.elem, expr, offset);
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
                        RefType::Box => (quote! { Box::new(move #closure) }, quote![], Destruct::Preserve(match fn_type {
                            FnTrait::FnMut => quote! { mut },
                            _ => quote! {},
                        })),
                        RefType::Mut => (quote! { &mut #closure }, quote![], Default::default()),
                        RefType::Ref => (quote! { & #closure }, quote![], Default::default()),
                        RefType::None => unreachable!("expect &dyn, &mut dyn or Box<dyn>"),
                    }
                }
            },
            _ => {},
        }
        (expr.to_token_stream(), quote![], Default::default())
    }
}
