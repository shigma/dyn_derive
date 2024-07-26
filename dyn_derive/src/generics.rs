use std::collections::HashMap;
use std::mem::replace;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::dyn_trait::get_full_name;

fn make_dyn(ident: &impl ToTokens, is_ref: bool) -> syn::Type {
    if is_ref {
        syn::parse_quote! { dyn #ident }
    } else {
        syn::parse_quote! { Box<dyn #ident> }
    }
}

// fixme: self argument
// fixme: separate static and dynamic args
struct Generic {
    param: syn::TypeParam,
    args: Vec<syn::Type>,
}

pub struct GenericsData {
    name: TokenStream,
    data: HashMap<String, Generic>,
}

impl GenericsData {
    pub fn from(inst: &mut syn::ItemTrait) -> Self {
        let mut data = HashMap::new();
        let params = replace(&mut inst.generics.params, Default::default());
        for param in params {
            let syn::GenericParam::Type(mut param) = param else {
                inst.generics.params.push(param);
                continue;
            };
            let index = param.attrs.iter().position(|attr| {
                attr.meta.path().is_ident("dynamic")
            });
            if let Some(index) = index {
                param.attrs.remove(index);
            } else {
                inst.generics.params.push(syn::GenericParam::Type(param));
                continue;
            }
            // todo: multiple bounds
            for bound in &mut param.bounds {
                match bound {
                    syn::TypeParamBound::Trait(bound) => {
                        let last = bound.path.segments.last_mut().unwrap();
                        last.ident = format_ident!("{}Instance", last.ident);
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
        }
        let (name, _) = get_full_name(inst);
        Self { name, data }
    }

    pub fn test(&self, path: &syn::TypePath, is_ref: bool) -> Option<(syn::Type, TokenStream)> {
        if path.qself.is_some() || path.path.segments.len() != 1 {
            return None
        }
        let last = path.path.segments.last().unwrap();
        if last.ident == "Self" {
            return Some((
                make_dyn(&self.name, is_ref),
                quote! { Self },
            ))
        }
        let Some(g) = self.data.get(&last.ident.to_string()) else {
            return None
        };
        let ident = &g.param.ident;
        let bounds = &g.param.bounds;
        let args = &g.args;
        return Some((
            make_dyn(bounds, is_ref),
            quote! { ::dyn_std::Instance::<#ident, (#(#args,)*)> },
        ))
    }
}
